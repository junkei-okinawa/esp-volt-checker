# ESP32 電圧チェッカー (Rust, esp-idf-hal, ADC)

ESP32 (M5Stack UnitCam) と `esp-idf-hal` を使用して、GPIO0 のアナログ電圧を測定する Rust プロジェクトです。

## 特徴

*   Rust と `esp-idf-hal` を使用
*   `esp-idf-hal` の ADC (One-shot) ドライバーを利用
*   [M5Stack UnitCam](https://docs.m5stack.com/en/unit/unit_cam) で動作確認済み
*   ADC キャリブレーション (Line Fitting) を使用して測定精度を向上
*   測定電圧を mV と観測範囲内のパーセンテージで表示

## 使用デバイス

*   **[M5Stack UnitCam](https://docs.m5stack.com/en/unit/unit_cam):** このプロジェクトは M5Stack UnitCam での使用を想定しています。UnitCam は利用可能な GPIO ピンに制限があるため、ADC2 のチャンネルとして **GPIO0** を使用しています。他の ESP32 ボードでも動作する可能性がありますが、ピン設定の変更が必要になる場合があります。

## 必要なもの

*   M5Stack UnitCam
*   Rust と ESP-IDF 開発環境 (espup を使用したセットアップ推奨)
*   測定対象の電圧源
*   (任意) 1kΩ 程度のプルダウン抵抗 (ピンが未接続の場合の読み取り値を安定させるため)

## 配線

*   測定したい電圧源のプラス側を UnitCam の **GPIO0** (Grove ポートの G16 ピンに対応) に接続します。
*   測定したい電圧源のマイナス側 (GND) を UnitCam の **GND** (Grove ポートの GND ピンに対応) に接続します。
*   (任意) GPIO0 と GND の間に 1kΩ 程度のプルダウン抵抗を接続します。

**注意:**
*   GPIO0 に入力できる電圧範囲は、`src/main.rs` 内の `attenuation` 設定に依存します。デフォルト (`DB_11`) では約 0V ~ 3.6V です。**この範囲を超える電圧を入力しないでください。** ESP32 の ADC ピンは 3.3V までしか許容できません。より高い電圧を測定する場合は、分圧回路などを適切に設計してください。
*   M5Stack UnitCam の Grove ポートのピン配置は [公式ドキュメント](https://docs.m5stack.com/en/unit/unit_cam) を参照してください。
*   GPIO ピンが未接続 (フローティング) の場合、ADC の読み取り値は不安定になる可能性があります。プルダウン抵抗を接続することで、未接続時の読み取り値を GND レベル (またはそれに近い値) に安定させることができます。

## セットアップ

1.  **リポジトリをクローン:**
    ```bash
    git clone <repository-url>
    cd esp-volt-checker
    ```

2.  **コードの確認 (任意):**
    `src/main.rs` 内の ADC 設定 (`attenuation`, `calibration`) や、パーセンテージ計算に使用する電圧範囲 (`MIN_MV`, `MAX_MV`) を必要に応じて確認・変更してください。

## ビルドと書き込み

1.  **ビルド:**
    ```bash
    cargo build --release
    ```

2.  **書き込み & モニター:**
    UnitCam を接続し、シリアルポートを確認して以下のコマンドを実行します (`/dev/tty.usbserial-XXXXXX` は実際のポート名に置き換えてください)。
    ```bash
    cargo espflash flash --release --port /dev/tty.usbserial-XXXXXX --monitor
    ```

    モニターには、測定された電圧値 (mV) と、観測された範囲内でのパーセンテージが定期的に表示されます。例: `Voltage: 1500 mV (45.2%)`

## コードについて

*   `src/main.rs` に主要なロジックが含まれています。
*   `esp_idf_hal::adc::oneshot::AdcDriver` と `AdcChannelDriver` を使用して ADC を初期化・設定します。
*   `AdcChannelConfig` で減衰レベル (`attenuation`) とキャリブレーション方法 (`calibration`) を設定します。ESP32 では `Calibration::Line` が利用可能です。
*   `adc_channel.read()` メソッドでキャリブレーション済みの電圧値 (mV) を読み取ります。
*   読み取った電圧値を、観測された有効測定範囲 (`MIN_MV` から `MAX_MV`) 内のパーセンテージに変換して表示します。
    *   現在のコードでは、UnitCam の GPIO0 に 1kΩ のプルダウン抵抗を接続した状態で観測された範囲 [128mV, 3130mV] を使用しています。
    *   `MIN_MV` はピン未接続時の読み取り値、`MAX_MV` は 3.3V 入力時の読み取り値に対応します。
    *   この範囲は、使用する ESP32 モジュール、ピン、接続する抵抗などによって変動する可能性があります。必要に応じて `src/main.rs` 内の `MIN_MV` と `MAX_MV` の値を調整してください。

# Tarkov-MarketObserver

Tarkov-MarketObserverは、Escape from Tarkov (EFT) のアイテムのトレーダーとの取引額を取得し表示するDiscordボットです。現在はビットコインのトレーダーとの取引額を表示する機能があります。

## 機能

- ビットコインのトレーダーとの取引額の取得と表示。

## 今後のアップデート

- 様々なアイテムのトレーダーとの取引額の取得と表示機能の追加。
- フリーマーケットのアイテム価格情報の取得と表示機能の追加。

## 開発環境

- rust:1.71

### セットアップ手順:

1. RustとCargoをインストールします。
2. リポジトリをクローンします:
    ```sh
    git clone https://github.com/root309/tarkov_market_observer.git
    ```
3. ディレクトリに移動します:
    ```sh
    cd tarkov_market_observer
    ```
4. 依存関係をインストールします:
    ```sh
    cargo build
    ```
5. 環境変数を設定します

### `.env` example

1. [Discord Developer Portal](https://discord.com/developers/applications) にアクセスして、新しいアプリケーションを作成します。
2. "Bot" タブをクリックして新しいボットを作成し、トークンをコピーします。
3. プロジェクトのルートディレクトリに`.env`ファイルを作成し、以下の形式でトークンを追加します:
    ```sh
    DISCORD_TOKEN=YOUR_DISCORD_BOT_TOKEN
    ```
4. `YOUR_DISCORD_BOT_TOKEN`を、2.でコピーしたトークンに置き換えます。

### 実行方法:
1. プロジェクトのルートディレクトリで以下のコマンドを実行します:
    ```sh
    cargo run
    ```

## 使い方

Discordで`!btc`コマンドを使用すると、ビットコインの現在のトレーダーとの取引額を取得し表示します。

## 開発者へ

プルリクエストは大歓迎です。大きな変更については、まずissueを開き、変更したい点を議論してください。

## ライセンス

[MIT License](LICENSE)

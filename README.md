# Tarkov-MarketObserver

Tarkov-MarketObserverは、Escape from Tarkov (EFT) のアイテムのトレーダーとの取引額を取得し表示するDiscordボットです。現在はビットコインのトレーダーとの取引額を表示する機能があります。

## 機能

- ビットコインのトレーダーとの取引額の取得と表示。

## 今後のアップデート

- 様々なアイテムのトレーダーとの取引額の取得と表示機能の追加。
- フリーマーケットのアイテム価格情報の取得と表示機能の追加。

## 使い方

Discordで`!btc`コマンドを使用すると、ビットコインの現在のトレーダーとの取引額を取得し表示します。

### トークンの設定

1. [Discord Developer Portal](https://discord.com/developers/applications) にアクセスして、新しいアプリケーションを作成します。
2. "Bot" タブをクリックして新しいボットを作成し、トークンをコピーします。
3. プロジェクトのルートディレクトリに`.env`ファイルを作成し、以下の形式でトークンを追加します:
    ```sh
    DISCORD_TOKEN=YOUR_DISCORD_BOT_TOKEN
    ```
4. `YOUR_DISCORD_BOT_TOKEN`を、2.でコピーしたトークンに置き換えます。
## 開発者へ

プルリクエストは大歓迎です。大きな変更については、まずissueを開き、変更したい点を議論してください。

## ライセンス

[MIT License](LICENSE)
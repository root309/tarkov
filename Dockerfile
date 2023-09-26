# ビルドステージ
FROM rust:1 as builder

# アプリケーションのディレクトリを作成
WORKDIR /usr/src/Tarkov-MarketObserver

# 依存関係のためのファイルをコピー
COPY Cargo.toml Cargo.lock ./

# 依存関係をビルド（依存関係のキャッシュを改善）
RUN cargo install cargo-build-deps && cargo build-deps --release

# ソースコードをコピー
COPY src ./src

# アプリケーションをビルド
RUN cargo install --path .

# ランタイムステージ
FROM debian:bullseye-slim

# アプリケーションのディレクトリを作成
WORKDIR /app

# ビルダーステージからビルド済みのバイナリをコピー
COPY --from=builder /usr/local/cargo/bin/tarkov_market_observer .

# アプリケーションを実行
ENTRYPOINT ["./tarkov_market_observer"]

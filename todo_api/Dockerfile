# Rustのイメージをベースに使用
FROM rust:latest

# 作業ディレクトリを設定
WORKDIR /usr/src/todo_api

# Rustのコンパイルに必要なファイルをコピー
COPY Cargo.toml Cargo.lock ./

# ビルドのキャッシュレイヤーを作成するためにダミーのソースファイルを作成
RUN mkdir src && \
    echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs

# 依存関係だけを先にビルド
RUN cargo build --release

# 実際のソースコードをコピー
COPY . .

# 再度ビルドを実行し、変更を反映
RUN touch src/main.rs && \
    cargo build --release

# コンテナ起動時にアプリケーションを実行
CMD ["cargo", "run", "--release"]

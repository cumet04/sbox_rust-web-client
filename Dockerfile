# 雑メモ: ローカル環境ではビルドしたのちに
# $ docker run -p 9000:8080 hoge
# $ curl -d '{}' "http://localhost:9000/2015-03-31/functions/function/invocations"
# のようにして動かせる

FROM rust:1.79 as builder
WORKDIR /app

COPY Cargo.toml .
COPY Cargo.lock .
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

COPY src src
RUN cargo build --release

FROM public.ecr.aws/lambda/provided:al2023
COPY --from=builder /app/target/release/sbox_rust-web-client ${LAMBDA_RUNTIME_DIR}/bootstrap
CMD [ "lambda-handler" ]

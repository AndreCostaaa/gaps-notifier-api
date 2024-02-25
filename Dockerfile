
FROM rust:1-buster AS builder


WORKDIR /app

COPY Cargo.toml .
COPY src src

RUN cargo build --release

FROM rust:1-buster as release

EXPOSE 3000

WORKDIR /app

COPY --from=builder /app/target/release/gaps-notifier-api .


CMD ["./gaps-notifier-api"]



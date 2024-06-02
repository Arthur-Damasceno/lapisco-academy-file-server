FROM rust:1.78 as builder

WORKDIR /app

COPY . .

RUN cargo install sqlx-cli

RUN sqlx database create -D sqlite:local.db
RUN sqlx migrate run -D sqlite:local.db

RUN cargo build --release

FROM debian:12-slim

WORKDIR /app

COPY --from=builder /app/.env /app/.env
COPY --from=builder /app/local.db /app/local.db
COPY --from=builder /app/target/release/lafs /app/lafs

EXPOSE 3000

CMD ["./lafs"]

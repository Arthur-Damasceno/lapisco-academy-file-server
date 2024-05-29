FROM rust:1.78

WORKDIR /app

COPY . .

RUN cargo install sqlx-cli

RUN sqlx database create -D sqlite:local.db
RUN sqlx migrate run -D sqlite:local.db

RUN cargo install --path .

EXPOSE 3000

CMD ["lafs"]

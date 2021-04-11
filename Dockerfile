FROM rust:latest

WORKDIR /app

RUN rustup default nightly

COPY . .

EXPOSE 8080

ENV ROCKET_ENV production

RUN cargo build

ENTRYPOINT ["cargo", "run"]
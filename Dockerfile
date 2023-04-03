FROM rust:latest as builder

WORKDIR /
COPY . .

RUN cargo install --path .

FROM debian:buster-slim
COPY --from=builder target/release/icountant-api /usr/local/bin/icountant-api
EXPOSE 8080
CMD ["icountant-api"]

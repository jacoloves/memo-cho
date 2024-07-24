FROM rust:1.64 as builder

WORKDIR /usr/src/memo-cho
COPY . .

RUN cargo install --path .

FROM debian:buster-slim

RUN apt-get update && apt-get install -y nano vim

COPY --from=builder /usr/local/cargo/bin/memo-cho /usr/local/bin/memo-cho

ENTRYPOINT [ "emmo-cho" ]
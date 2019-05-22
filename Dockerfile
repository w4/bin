FROM rust:1.34.2-slim-stretch AS builder
RUN rustup install nightly-x86_64-unknown-linux-gnu

COPY . /sources
WORKDIR /sources
RUN cargo +nightly build --release
RUN chown nobody:nogroup /sources/target/release/bin


FROM debian:stretch-slim
COPY --from=builder /sources/target/release/bin /pastebin

USER nobody
EXPOSE 8000
ENTRYPOINT ["/pastebin"]

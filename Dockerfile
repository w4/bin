FROM rust:1-slim AS builder

RUN apt-get update && \
    apt-get install -y libclang-dev musl-tools
RUN rustup target add x86_64-unknown-linux-musl

COPY . /sources
WORKDIR /sources
# force static linking with target to avoid glibc issues
RUN cargo build --release --target x86_64-unknown-linux-musl
RUN chown nobody:nogroup /sources/target/x86_64-unknown-linux-musl/release/bin

FROM alpine:latest
COPY --from=builder /sources/target/x86_64-unknown-linux-musl/release/bin /pastebin

USER nobody
EXPOSE 8000
ENTRYPOINT ["/pastebin", "0.0.0.0:8000"]

FROM rust:1-slim-bookworm AS builder

RUN apt update && apt install -y libclang-dev

COPY . /sources
WORKDIR /sources
RUN cargo build --release
RUN chown nobody:nogroup /sources/target/release/bin

FROM gcr.io/distroless/cc-debian12
COPY --from=builder /sources/target/release/bin /pastebin

USER nobody
EXPOSE 8000
ENTRYPOINT ["/pastebin", "0.0.0.0:8000"]

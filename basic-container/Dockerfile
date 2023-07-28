FROM rust:slim-bookworm AS builder

WORKDIR /src
COPY . .

RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /src/target/release/basic-container /basic-container
CMD [ "/basic-container" ]
FROM rust:alpine3.18 AS builder

WORKDIR /src
RUN rustup target add x86_64-unknown-linux-musl
COPY . .

RUN cargo build  --target x86_64-unknown-linux-musl --release

FROM scratch
COPY --from=builder /src/target/x86_64-unknown-linux-musl/release/basic-container-scratch /basic-container-scratch
CMD [ "/basic-container-scratch" ]
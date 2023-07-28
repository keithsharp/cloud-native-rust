FROM rust:alpine3.18 AS builder

WORKDIR /src
COPY . .

RUN cargo build --release

FROM alpine:3.18
COPY --from=builder /src/target/release/basic-container-alpine /basic-container-alpine
CMD [ "/basic-container-alpine" ]
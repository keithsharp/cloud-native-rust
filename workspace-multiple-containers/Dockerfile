FROM rust:alpine3.18 AS builder

WORKDIR /src
RUN rustup target add x86_64-unknown-linux-musl
COPY . .

RUN cargo build --target x86_64-unknown-linux-musl --release

FROM scratch as one
COPY --from=builder /src/target/x86_64-unknown-linux-musl/release/one /one
CMD [ "/one" ]
LABEL service=one

FROM scratch as two
COPY --from=builder /src/target/x86_64-unknown-linux-musl/release/two /two
CMD [ "/two" ]
LABEL service=two
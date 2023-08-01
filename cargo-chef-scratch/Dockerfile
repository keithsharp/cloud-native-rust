FROM rust:slim-bookworm as chef
RUN apt-get update && apt-get install -y musl-tools
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo install cargo-chef
WORKDIR /src

FROM chef as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /src/recipe.json recipe.json
RUN cargo chef cook --target=x86_64-unknown-linux-musl --release --recipe-path=recipe.json
COPY . .
RUN cargo build --target=x86_64-unknown-linux-musl --release

FROM scratch as chef-one
COPY --from=builder /src/target/x86_64-unknown-linux-musl/release/one /one
CMD [ "/one" ]
LABEL service=chef-one

FROM scratch as chef-two
COPY --from=builder /src/target/x86_64-unknown-linux-musl/release/two /two
CMD [ "/two" ]
LABEL service=chef-two
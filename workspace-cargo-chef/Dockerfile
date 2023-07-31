FROM rust:slim-bookworm as chef
RUN apt-get update
RUN apt-get install pkg-config libssl-dev -y
RUN cargo install cargo-chef
WORKDIR /src

FROM chef as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /src/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path=recipe.json
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim as chef-one
RUN apt-get update
RUN apt-get install openssl ca-certificates -y
COPY --from=builder /src/target/release/one /one
CMD [ "/one" ]
LABEL service=chef-one

FROM debian:bookworm-slim as chef-two
RUN apt-get update
RUN apt-get install openssl ca-certificates -y
COPY --from=builder /src/target/release/two /two
CMD [ "/two" ]
LABEL service=chef-two
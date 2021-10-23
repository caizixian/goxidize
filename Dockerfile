FROM rust:alpine as builder
WORKDIR goxide
COPY . .
RUN apk add --no-cache musl-dev
RUN cargo build --release

FROM rust:alpine as runtime
WORKDIR goxide
COPY --from=builder goxide/target/release/goxide /usr/local/bin
CMD ["goxide"]

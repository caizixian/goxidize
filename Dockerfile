FROM rust:alpine as builder
WORKDIR goxidize
COPY . .
RUN apk add --no-cache musl-dev
RUN cargo build --release

FROM rust:alpine as runtime
WORKDIR goxidize
COPY --from=builder goxidize/target/release/goxidize /usr/local/bin
CMD ["goxidize"]

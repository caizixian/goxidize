# syntax=docker/dockerfile:1
FROM alpine:3
WORKDIR /goxidize
COPY ./target/x86_64-unknown-linux-musl/release/goxidize /goxidize/goxidize
COPY ./dist /goxidize/dist
ENTRYPOINT ["/goxidize/goxidize"]

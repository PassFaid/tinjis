# -*- mode: dockerfile -*-
# build release executable
ARG BASE_IMAGE=ekidd/rust-musl-builder:latest

FROM ${BASE_IMAGE} AS builder

ADD --chown=rust:rust . ./

RUN cargo build --release

# copy to alpine image
FROM alpine:latest
RUN apk --no-cache add ca-certificates
COPY --from=builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/provider \
    /usr/local/bin/
CMD /usr/local/bin/provider

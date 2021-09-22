FROM rust:1.55-alpine3.14 as builder
COPY whatever /code/webapp
WORKDIR /code/webapp
RUN cargo install --path .

FROM alpine:3.14
COPY --from=builder /usr/local/cargo/bin/webapp /usr/local/bin/webapp
CMD ["webapp"]

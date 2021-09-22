FROM rust:1.55-alpine3.14 as builder
COPY webapp /code/webapp
WORKDIR /code/webapp
RUN cargo build --release 

FROM alpine:3.14
COPY --from=builder /code/webapp/target/release/webapp /usr/local/bin/webapp
CMD ["webapp"]

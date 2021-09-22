FROM rust:1.55-slim-buster as builder
COPY webapp /code/webapp
WORKDIR /code/webapp
RUN cargo build --release 

FROM rust:1.55-slim-buster
COPY --from=builder /code/webapp/target/release/webapp /usr/local/bin/webapp
CMD ["webapp"]

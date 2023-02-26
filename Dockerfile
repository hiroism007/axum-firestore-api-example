# build stage
FROM rust:1.67.0 as builder
WORKDIR /usr/src/firebase-auth-api
COPY . .
RUN cargo install --path .


# runtime stage
FROM debian:buster-slim
RUN apt-get update & apt-get install -y extra-runtime-dependencies & rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/firebase-auth-api /usr/local/bin/firebase-auth-api
CMD ["firebase-auth-api"]

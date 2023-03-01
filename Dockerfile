# builder
FROM rust:1.67.0 as builder
WORKDIR /usr/src/firebase-auth-api
COPY . .
RUN cargo install --path .

# distroless
FROM gcr.io/distroless/cc
COPY --from=builder /usr/local/cargo/bin/firebase-auth-api /usr/local/bin/firebase-auth-api
CMD ["firebase-auth-api"]

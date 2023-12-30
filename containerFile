FROM rust:alpine AS builder
RUN apk add musl-dev
WORKDIR /app
COPY . .
RUN cargo build --release

FROM alpine:latest
WORKDIR /app
COPY --from=builder /app/target/release/website .
CMD ["./website"]

# Build stage
FROM rust:1.75-alpine AS builder

RUN apk add --no-cache musl-dev openssl-dev openssl-libs-static pkgconfig

WORKDIR /app

# Copy the server code
COPY server/ ./

# Build the application
RUN cargo build --release

# Runtime stage
FROM alpine:3.19

RUN apk add --no-cache ca-certificates

WORKDIR /app

# Copy the binary from the build stage
COPY --from=builder /app/target/release/airi-press-server /app/airi-press-server

# Copy migrations
COPY --from=builder /app/migrations /app/migrations

# Expose the port
EXPOSE 3000

# Run the binary
CMD ["/app/airi-press-server"]

# Build stage
FROM rust:1.85-alpine AS builder

RUN apk add --no-cache musl-dev openssl-dev openssl-libs-static pkgconfig

WORKDIR /app

# Copy the server code
COPY server/ ./

# Downgrade incompatible dependencies to versions supporting rustc 1.85
RUN cargo update home@0.5.12 --precise 0.5.9 && \
    cargo update time@0.3.47 --precise 0.3.36 && \
    cargo update time-core@0.1.8 --precise 0.1.2 && \
    cargo update time-macros@0.2.27 --precise 0.2.18

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

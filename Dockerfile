# Stage 1: Build the Rust application
FROM rust:latest as builder

WORKDIR /app
COPY . .

RUN cargo build --release

# Stage 2: Create a distroless container to run the app
FROM gcr.io/distroless/cc

WORKDIR /root/
COPY --from=builder /app/target/release/fibbot .

ENTRYPOINT ["/root/fibbot-action"]

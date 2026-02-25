# Build stage
FROM rust:1.93-slim as builder

WORKDIR /app

# Install dependencies
RUN apt-get update && apt-get install -y \
  pkg-config \
  libssl-dev \
  libsqlite3-dev \
  && rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src
COPY migrations ./migrations
COPY dist ./dist

# Build for release
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
  ca-certificates \
  libssl3 \
  libsqlite3-0 \
  && rm -rf /var/lib/apt/lists/*

# Copy the binary from builder
COPY --from=builder /app/target/release/feathers_and_fairways /app/feathers_and_fairways

# Copy migrations and static files
COPY --from=builder /app/migrations /app/migrations
COPY --from=builder /app/dist /app/dist

# Expose port
EXPOSE 3000

# Run the application
CMD ["/app/feathers_and_fairways"]

# --- Stage 1: Build the Next.js Frontend ---
# Use an official Node.js image as the base
FROM node:24-alpine AS frontend-builder
WORKDIR /app

# Copy package files and install dependencies
COPY web/package.json web/package-lock.json ./web/
RUN npm ci --prefix web

# Copy the rest of the frontend source code and build
COPY web/ ./web/
RUN npm run build --prefix web
# The static files are now in /app/frontend/out


# --- Stage 2: Build the Rust Backend ---
# Use the official Rust image
FROM rust:1.91-alpine AS backend-builder
WORKDIR /app

# Install build dependencies for Alpine
RUN apk add --no-cache \
    musl-dev \
    build-base \
    libpq-dev \
    openssl-dev \
    openssl-libs-static \
    zlib-static \
    pkgconfig \
    perl

# Set environment variable to force OpenSSL to link statically
ENV RUSTFLAGS="-C target-feature=-crt-static"
#ENV OPENSSL_STATIC=1
#ENV OPENSSL_LIB_DIR=/usr/lib
#ENV OPENSSL_INCLUDE_DIR=/usr/include

# Create a dummy project to cache dependencies
COPY Cargo.toml Cargo.lock ./

RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release

# Copy the real source code and build
COPY src ./src
COPY migrations ./migrations
# Touching main.rs ensures cargo rebuilds instead of using the dummy binary
RUN touch src/main.rs && \
    cargo build --release

# The final binary is at /app/target/release/<your-binary-name>
# We assume the binary name is 'backend' based on Cargo.toml [package].name


# --- Stage 3: Create the Final Production Image ---
# Use a minimal Alpine image
FROM alpine:3.20
WORKDIR /app

# Install runtime dependencies (libpq is required for Postgres)
#RUN apk add --no-cache libpq ca-certificates

# Copy the static frontend files from the 'frontend-builder' stage
# We place them in a 'static' directory for Axum to serve
COPY --from=frontend-builder /app/web/out ./static

# Copy the compiled Rust binary from the 'backend-builder' stage
# IMPORTANT: Change 'backend' to your actual binary name from Cargo.toml
COPY --from=backend-builder /app/target/release/backend .

# Expose the port your Axum server listens on
EXPOSE 8000

# Run the backend server
CMD ["./backend"]

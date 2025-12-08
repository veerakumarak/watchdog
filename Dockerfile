# --- Stage 1: Build the Next.js Frontend ---
# Use an official Node.js image as the base
FROM node:24-alpine AS frontend-builder
WORKDIR /app

# Copy package files and install dependencies
COPY web/package.json web/package-lock.json ./frontend/
RUN npm ci --prefix frontend

# Copy the rest of the frontend source code and build
COPY web/ ./frontend/
RUN npm run build --prefix frontend
# The static files are now in /app/frontend/out


# --- Stage 2: Build the Rust Backend ---
# Use the official Rust image
FROM rust:1.91-alpine AS backend-builder
WORKDIR /app

# Install build dependencies for Alpine
RUN apk add --no-cache musl-dev build-base

# Create a dummy project to cache dependencies
COPY backend/Cargo.toml backend/Cargo.lock ./backend/
RUN mkdir /app/backend/src && \
    echo "fn main() {}" > /app/backend/src/main.rs && \
    cargo build --release --manifest-path /app/backend/Cargo.toml

# Copy the real source code and build
COPY backend/src ./backend/src
RUN touch /app/backend/src/main.rs && \
    cargo build --release --manifest-path /app/backend/Cargo.toml

# The final binary is at /app/target/release/<your-binary-name>
# We assume the binary name is 'backend' based on Cargo.toml [package].name


# --- Stage 3: Create the Final Production Image ---
# Use a minimal Alpine image
FROM alpine:3.20
WORKDIR /app

# Copy the static frontend files from the 'frontend-builder' stage
# We place them in a 'static' directory for Axum to serve
COPY --from=frontend-builder /app/frontend/out ./static

# Copy the compiled Rust binary from the 'backend-builder' stage
# IMPORTANT: Change 'backend' to your actual binary name from Cargo.toml
COPY --from=backend-builder /app/target/release/backend .

# Expose the port your Axum server listens on
EXPOSE 8000

# Run the backend server
CMD ["./backend"]

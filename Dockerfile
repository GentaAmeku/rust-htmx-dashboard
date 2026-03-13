# Stage 1: CSS build
FROM node:22-alpine AS css-builder
WORKDIR /app
COPY package.json pnpm-lock.yaml ./
RUN corepack enable && pnpm install --frozen-lockfile
COPY assets/tailwind.css assets/tailwind.css
COPY templates/ templates/
RUN pnpm build:css

# Stage 2: Rust build
FROM rust:1.88-bookworm AS rust-builder
WORKDIR /app

# Dependency cache layer
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release && rm -rf src

# Build application
COPY src/ src/
COPY templates/ templates/
COPY migrations/ migrations/
ENV SQLX_OFFLINE=true
RUN touch src/main.rs && cargo build --release

# Stage 3: Runtime
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app

COPY --from=rust-builder /app/target/release/rust-htmx-dashboard .
COPY --from=css-builder /app/public/ public/
COPY templates/ templates/

EXPOSE 3000
CMD ["./rust-htmx-dashboard"]

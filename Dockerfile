# Этап 1: сборка Rust (используем rustls вместо OpenSSL)
FROM rust:1.86-slim AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock* ./
COPY crates ./crates
RUN cargo build --release

# Этап 2: финальный образ с Python
FROM python:3.11-slim
WORKDIR /app

COPY --from=builder /app/target/release/healdep /usr/local/bin/healdep
COPY python/requirements.txt ./python/
RUN pip install --no-cache-dir -r python/requirements.txt
COPY python/ ./python/
COPY web/ ./web/
COPY healdep.toml ./
COPY examples/ ./examples/

EXPOSE 5000
CMD ["python", "python/server.py"]

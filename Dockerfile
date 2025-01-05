# Étape 1 : Compilation
FROM rust:1.83 AS builder
COPY . .
WORKDIR /
RUN cargo build --release

# Étape 2 : Exécutable léger
FROM debian:buster-slim
WORKDIR /
COPY --from=builder ./target/release/gonecode-backend .
EXPOSE 8080
CMD ["./gonecode-backend"]
FROM rust:1.89.0 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/kittycat /usr/local/bin/kittycat
COPY --from=builder /app/web /opt/kittycat/web
WORKDIR /opt/kittycat
EXPOSE 8080
ENTRYPOINT ["kittycat"]
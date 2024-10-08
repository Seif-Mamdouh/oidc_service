FROM rust:1.76

WORKDIR /app

COPY . .

RUN cargo build --release

CMD ["./target/release/oidc-service"]

FROM rust:1.76 as builder

WORKDIR /app

# Copy only Cargo.toml and Cargo.lock first to cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# Now copy the rest of the source code and build the application
COPY . .
RUN touch src/main.rs && cargo build --release

# Final stage
FROM debian:buster-slim
COPY --from=builder /app/target/release/oidc-service /usr/local/bin/

CMD ["oidc-service"]

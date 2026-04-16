FROM rust:1.72 as builder

WORKDIR /app

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# Build the actual application
COPY . ./
RUN cargo build --release

FROM debian:buster-slim

WORKDIR /app

COPY --from=builder /app/target/release/data_privacy_vault ./

EXPOSE 8080

CMD ["./data_privacy_vault"]

FROM rust:1-trixie AS chef
RUN cargo install cargo-chef --locked
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release

FROM debian:trixie-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/services_miraki /app/
EXPOSE 8080
CMD ["/app/services_miraki"]

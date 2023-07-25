FROM clux/muslrust:stable AS chef
USER root
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl --bin economy-data-api

FROM alpine AS runtime
ENV PORT="5000"
EXPOSE 5000
RUN addgroup -S economy-data-api && adduser -S economy-data-api -G economy-data-api
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/economy-data-api /usr/local/bin/
USER economy-data-api
CMD ["/usr/local/bin/economy-data-api"]

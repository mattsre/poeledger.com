FROM clux/muslrust:stable AS builder
COPY Cargo.* .
COPY src/ src/
RUN ln -s /bin/g++ /bin/musl-g++
RUN --mount=type=cache,target=/volume/target \
    --mount=type=cache,target=/root/.cargo/registry \
    cargo build --release --bin price-history-api && \
    mv /volume/target/x86_64-unknown-linux-musl/release/price-history-api .

FROM cgr.dev/chainguard/static
COPY --from=builder --chown=nonroot:nonroot /volume/price-history-api /app/
ENTRYPOINT ["/app/price-history-api"]

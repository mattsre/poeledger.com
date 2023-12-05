FROM clux/muslrust:stable AS builder
COPY Cargo.* .
COPY src/ src/
RUN --mount=type=cache,target=/volume/target \
    --mount=type=cache,target=/root/.cargo/registry \
    cargo build --release --bin river-crawler && \
    mv /volume/target/x86_64-unknown-linux-musl/release/river-crawler .

FROM cgr.dev/chainguard/static
COPY --from=builder --chown=nonroot:nonroot /volume/river-crawler /app/
ENTRYPOINT ["/app/river-crawler"]

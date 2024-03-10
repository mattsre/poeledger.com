FROM clux/muslrust:stable AS builder
COPY Cargo.* .
COPY src/ src/
RUN ln -s /bin/g++ /bin/musl-g++
RUN --mount=type=cache,target=/volume/target \
    --mount=type=cache,target=/root/.cargo/registry \
    cargo build --release --bin stash-processor && \
    mv /volume/target/x86_64-unknown-linux-musl/release/stash-processor .

FROM cgr.dev/chainguard/static
COPY --from=builder --chown=nonroot:nonroot /volume/stash-processor /app/
ENTRYPOINT ["/app/stash-processor"]

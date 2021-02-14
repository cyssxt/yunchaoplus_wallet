FROM clux/muslrust:stable as base
COPY .cargo-config /root/.cargo/config
RUN mkdir -p /volume/target

FROM base as builder
WORKDIR /volume
COPY . .
RUN cargo build --release

FROM scratch
COPY --from=builder /volume/target/x86_64-unknown-linux-musl/release/yunchaoplus_rw /yunchaoplus_rw
ENV RUST_LOG=info
EXPOSE 8080
ENTRYPOINT ["/yunchaoplus_rw"]
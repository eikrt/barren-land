FROM rust:1.63
COPY ./ ./
RUN cargo build --bin generator
CMD ["./target/release/generator"]

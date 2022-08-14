FROM rust:alpine 
COPY ./ ./
RUN cargo build --bin server 
CMD ["./target/release/server"]

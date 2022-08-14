FROM rust:1.63
COPY ./ ./
RUN cargo build --bin client 
CMD ["./target/release/client" ,"user", "pass"]

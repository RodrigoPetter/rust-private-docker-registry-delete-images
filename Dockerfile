FROM rust
WORKDIR delete-images
COPY . .
RUN cargo build --release
CMD ./target/release/rust-private-docker-registry-delete-images
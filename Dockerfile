FROM rust

COPY ./ ./

RUN cargo build --release

EXPOSE 80

CMD ["./target/release/wings"]
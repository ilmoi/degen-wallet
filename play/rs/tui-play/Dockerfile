FROM rust:1.53
WORKDIR app
COPY . .
RUN cargo build
ENTRYPOINT ["./target/debug/tui_play"]
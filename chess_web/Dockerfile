FROM rust:slim-bullseye as base
WORKDIR /app
COPY . .
RUN cargo install --bins trunk
RUN rustup target add wasm32-unknown-unknown
ENV PORT=8080
CMD trunk serve --address 0.0.0.0 --port $PORT

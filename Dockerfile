FROM rust:slim-bullseye as base
WORKDIR /app
COPY . .
RUN cargo install --bins thunk
RUN rustup target add wasm32-unknown-unknown
CMD
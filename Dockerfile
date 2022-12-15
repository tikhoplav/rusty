FROM rustlang/rust:nightly-alpine
RUN apk add --no-cache \
		build-base \
  && rustup target add wasm32-unknown-unknown \
  && cargo install cargo-watch
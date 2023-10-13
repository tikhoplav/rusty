# Rusty

Rusty is the in-browser game engine written in Rust and compiled in WASM which
in conjunction with WebGL based canvas takes the gaming experience to the new
level.

<br>

> This project is temporary frozen until the [WebGPU](https://www.w3.org/TR/webgpu/)
> feature is stable at least for the `stable` linux chrome, as Rust engine module is
> supposed to work directly with the graphical API to exchange draw buffers.

<br>

## Testing

Unit tests of the rust code that is compiled into WASM module and it's benchmarks  
can be run with the following commands respectively:

```
docker compose run rust /bin/sh -c 'cargo test --target x86_64-unknown-linux-musl'
docker compose run rust /bin/sh -c 'cargo +nightly bench --target x86_64-unknown-linux-musl'
```

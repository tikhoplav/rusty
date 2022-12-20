## Testing

Unit tests of the rust code that is compiled into WASM module and it's benchmarks  
can be run with the following commands respectively:

```
docker compose run rust /bin/sh -c 'cargo test --target x86_64-unknown-linux-musl'
docker compose run rust /bin/sh -c 'cargo +nightly bench --target x86_64-unknown-linux-musl'
```

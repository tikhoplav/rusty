interface RustFFI {
  vec_data(ptr: number): number;
  vec_len(ptr: number): number;
}

interface Wasm extends RustFFI{
  memory: WebAssembly.Memory;
  [key: string]: any;
}

const WasmAPI = (wasm: Wasm) => ({
  vecBuffer: (ptr: number): [
    ArrayBuffer,
    number,
    number,
  ] => [
    wasm.memory.buffer,
    wasm.vec_data(ptr),
    wasm.vec_len(ptr),
  ],
})

require("../target/wasm32-unknown-unknown/release/rusty_client.wasm")
  .then((wasm: Wasm) => {
    const { vecBuffer } = WasmAPI(wasm);
    const decoder = new TextDecoder('utf-8');

    const greetPtr = wasm.greet();
    const greet = decoder.decode(
      new Uint8Array(...vecBuffer(greetPtr))
    );
    console.log(greetPtr, greet);
    wasm.destroy(greetPtr);

    const motdPtr = wasm.motd();
    const motd = decoder.decode(
      new Uint8Array(...vecBuffer(motdPtr))
    );
    console.log(motdPtr, motd);
    wasm.destroy(motdPtr);

    const genPtr = wasm.gen();
    const gen = new Uint32Array(...vecBuffer(genPtr));
    console.log(gen);
  })
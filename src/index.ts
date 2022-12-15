import Rusty, { loadModule } from "./rusty";

// require("../target/wasm32-unknown-unknown/release/rusty_client.wasm")
//   .then((wasm: Module) => {
//     const rusty = new Rusty(wasm);

//     const decoder = new TextDecoder('utf-8');

//     const greetPtr = (wasm.greet as Function)();
//     const greet = decoder.decode(
//       new Uint8Array(...rusty.vecBuffer(greetPtr))
//     );
//     console.log(greetPtr, greet);
//     (wasm.destroy as Function)(greetPtr);

//     const motdPtr = (wasm.motd as Function)();
//     const motd = decoder.decode(
//       new Uint8Array(...rusty.vecBuffer(motdPtr))
//     );
//     console.log(motdPtr, motd);
//     (wasm.destroy as Function)(motdPtr);

//     const genPtr = (wasm.gen as Function)();
//     const gen = new Uint32Array(...rusty.vecBuffer(genPtr));
//     console.log(gen);
//   })

// const rusty = require("../target/wasm32-unknown-unknown/release/rusty_client.wasm");
// 

loadModule("rusty.wasm")
  .then(rusty => {    
    console.log(rusty.greet());
  })

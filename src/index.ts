import { loadModule } from "./rusty";

loadModule("rusty.wasm")
  .then(rusty => {    
    console.log(rusty.greet());
  })

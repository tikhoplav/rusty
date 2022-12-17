# Dev Note

## Data transfer from WSAM into WebGL.

There are many different aspects and problems that need to be solved in order  
to produce an efficient game renderer. One of them is a data transfer. Let's  
take for example vertex data. Regular game scene may contain ten to thousand  
objects. Let's assume that we are rendering sprite animation, and each sprite  
is a quad with texture and uv coordinates. That alone result in 4 floats per  
vertex -> 16 float numbers per quad -> 64 bytes per quad. In total that would  
take from 640b to 6.4Kb of raw data for each frame to be rendered. That number  
is much higher for 3D rigged meshes with skeletal animations e.t.c.

Since that data can't be stored and managed at the GPU side on it's own it is  
required for the game engine to pass that data into GPU before each draw call.  
The more time is spent for data transferring from RAM to GPU memory the less  
time the renderer have to actually draw a scene. That's why it is crucial to  
solve memory transferring problem as early in development as possible.

Comparing JS and Rust/WASM it is obvious that WASM should keep and manage the  
memory that contains all the game data used for rendering. This brings another  
requirement: **WASM heap memory should be passed directly to WebGL**. **Directly**  
means that the same exact bytes that WASM use to store and operate game state  
should be transferred to the graphics memory without any intermediate copies or  
allocations.

```
// Create a direct view to WASM memory
const buffer = new Uint8Array(wasm.memory.buffer, _start, _length)

// Pass data to WebGL
gl.bufferSubData(gl.ARRAY_BUFFER, _offset, buffer)
```

However, that's not all. During runtime WASM module memory can be `grown` what  
leads to the existing buffer (`wasm.memory.buffer`) become detached and can no  
longer be used. There are two ways to maintain a working buffer:

- Preallocate enough space in the module and never let it grow;
- Keep track of memory events and refresh the buffer;
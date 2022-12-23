export async function loadModule(src: string): Promise<Module> {
  const { instance }: WebAssembly.WebAssemblyInstantiatedSource = await WebAssembly.instantiateStreaming(fetch(src), {});
  return new Module(instance);
}

export class Module {
  private memory: WebAssembly.Memory;
  private ffi: { [key: string]: Function };

  constructor (wasm: WebAssembly.Instance) {
    const { memory, ...ffi } = wasm.exports;
    this.memory = memory as WebAssembly.Memory;
    this.ffi = ffi as { [key: string]: Function };
  }

  /**
   * Provides a low level access to the `Vec` bytes.
   * Supposed to be used with Array Buffer constructor, example:
   * 
   * ```
   * const vectorData = new Uint8Array(...rusty.vecBuffer(vecPtr));
   * ```
   * 
   * @param ptr 
   * @returns 
   */
  private vecBuffer(ptr: number): [
    ArrayBuffer,
    number,
    number,
  ] {
    return [
      this.memory.buffer,
      this.ffi.vec_data(ptr),
      this.ffi.vec_len(ptr),
    ]
  }

  greet(): string {
    return new TextDecoder('utf-8').decode(
      new Uint8Array(...this.vecBuffer(this.ffi.greet()))
    );
  }

  initState() {
    this.ffi.gen()
  }

  update() {
    this.ffi.update()
  }

  set screen({ width, height }: {
    width: number,
    height: number,
  }) {
    this.ffi.set_screen_width(width)
    this.ffi.set_screen_height(height)
  }

  get viewMatrix(): Float32Array {
    return new Float32Array(this.memory.buffer, this.ffi.state_view_matrix(), 16)
  }

  get verticesData(): ArrayBuffer {
    return new Uint8Array(this.memory.buffer, this.ffi.state_data(), this.ffi.state_len() * 16)
  }

  rotateCamera(pixels: number): number {
    return this.ffi.rotate_camera(pixels)
  }
};
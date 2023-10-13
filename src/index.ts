import { loadModule } from "./rusty";
import { makeShader } from "utils/gl";
import { loadBinary } from "utils/import";

const canvas = document.createElement('canvas')
document.querySelector('body').append(canvas)
const gl = canvas.getContext("webgl2");
// gl.enable(gl.CULL_FACE);

const spriteRenderer = function (gl: WebGL2RenderingContext) {
  const vertexShaderSource = `#version 300 es
uniform mat4 uMatrix;

layout (location = 0) in vec4 aPosition;
layout (location = 1) in vec4 AColor;

out vec4 vColor;

void main() {
  gl_Position = uMatrix * aPosition;
  vColor = AColor;
}`

  const fragmentShaderSource = `#version 300 es
precision highp float;

in vec4 vColor;
out vec4 fragColor;

void main() {
  fragColor = vColor;
}`

  const program = makeShader(gl, vertexShaderSource, fragmentShaderSource)
  gl.useProgram(program)

  const vertexBuffer = gl.createBuffer()
  gl.bindBuffer(gl.ARRAY_BUFFER, vertexBuffer)
  gl.bufferData(gl.ARRAY_BUFFER, 16 * 11808, gl.DYNAMIC_DRAW)

  // const vao = gl.createVertexArray()
  // gl.bindVertexArray(vao)

  // Setup position attribute, should read from buffer
  // as pairs of float coordinates. Total bytes per vertex is 12.
  gl.vertexAttribPointer(0, 3, gl.FLOAT, false, 0, 0)
  gl.enableVertexAttribArray(0)


  const colorBuffer = gl.createBuffer()
  gl.bindBuffer(gl.ARRAY_BUFFER, colorBuffer)
  gl.bufferData(gl.ARRAY_BUFFER, 16 * 11808, gl.DYNAMIC_DRAW)
  gl.vertexAttribPointer(1, 4, gl.UNSIGNED_SHORT, true, 0, 0)
  gl.enableVertexAttribArray(1)

  // // Setup color attribute, should read from buffer1024 as a vector
  // // of 4 bytes. Total bytes per vertex is 12.
  // gl.vertexAttribPointer(1, 4, gl.UNSIGNED_BYTE, true, 16, 12)
  // gl.enableVertexAttribArray(1)

  const indexBuffer = gl.createBuffer();
  gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, indexBuffer);
  gl.bufferData(gl.ELEMENT_ARRAY_BUFFER, 8 * 11808, gl.STATIC_DRAW);

  function update(
    viewMatrix: Float32Array,
    idx: ArrayBuffer,
    verts: ArrayBuffer,
    colors: ArrayBuffer,
  ): void {
    gl.useProgram(program)
    gl.uniformMatrix4fv(gl.getUniformLocation(program, "uMatrix"), false, viewMatrix)

    gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, indexBuffer)
    gl.bufferSubData(gl.ELEMENT_ARRAY_BUFFER, 0, idx)

    gl.bindBuffer(gl.ARRAY_BUFFER, vertexBuffer)
    // gl.bindVertexArray(vao)
    gl.bufferSubData(gl.ARRAY_BUFFER, 0, verts)

    gl.bindBuffer(gl.ARRAY_BUFFER, colorBuffer)
    gl.bufferSubData(gl.ARRAY_BUFFER, 0, colors)
  }

  return { update }
}

async function main() {
  const rusty = await loadModule("rusty.wasm")

  // const mesh = await loadBinary("assets/models/crate.bin")
  // const col = new Uint8Array(mesh, 0, 3584)
  // const data = new Uint8Array(mesh, 3584, 2688)
  // const norm = new Uint8Array(mesh, 6272, 2688)
  // const idx = new Uint8Array(mesh, 8960, 648)

  // const mesh = await loadBinary("assets/models/suzanne.bin")
  // // const col = new Uint8Array()
  // const data = new Uint8Array(mesh, 23616, 141696)
  // // const norm = new Uint8Array(mesh, 165312, 141696)
  // const col = new Uint8Array(mesh, 165312, 141696)
  // const idx = new Uint8Array(mesh, 0, 23616)
  
  const cube = await loadBinary("assets/models/tris.bin")
  const col = new Uint8Array(cube, 0, 72)
  const data = new Uint8Array(cube, 72, 108)
  const norm = new Uint8Array(cube, 180, 108)
  const idx = new Uint8Array(cube, 288, 18)

  // {
  //   const _col = new Uint16Array(cube, 0, 72 / 2)
  //   const _pos = new Float32Array(cube, 72, 108 / 4)
  //   const _norm = new Float32Array(cube, 180, 108 / 4)
  //   const _idx = new Uint16Array(cube, 288, 9)

  //   const vertices = [];

  //   for (let i = 0; i < 9; i++) {
  //     vertices.push({
  //       pos: [_pos[i*3], _pos[i*3 + 1], _pos[i*3 + 2]],
  //       col: [_col[i*4], _col[i*4 + 1], _col[i*4 + 2], _col[i*4 + 3]],
  //     })
  //   }

  //   console.log(vertices, Array.from(_idx))
  // }

  // console.log(
  //   new Uint8Array(cube, 72 + 6 * 4 * 3, 4 * 3)
  // )

  // const cube = await loadBinary("assets/models/prism.bin")
  // // const col = new Uint16Array(cube, 0, 48)
  // const data = new Uint8Array(cube, 96, 144)
  // // const norm = new Uint8Array(cube, 240, 144)
  // const col = new Uint8Array(cube, 240, 144)
  // const idx = new Uint8Array(cube, 384, 24)

  // const cube = await loadBinary("assets/models/cube.bin")
  // // const col = new Uint8Array(cube, 0, 192)
  // const data = new Uint8Array(cube, 0, 288)
  // // const norm = new Uint8Array(cube, 480, 288)
  // const col = new Uint8Array(cube, 480, 288)
  // const idx = new Uint8Array(cube, 1152, 72)

  rusty.initState()
  const viewMatrix = rusty.viewMatrix;
  // const data = rusty.verticesData;

  const { update } = spriteRenderer(gl)

  const onresize = () => {
    const { innerWidth: width, innerHeight: height} = window
    gl.canvas.width = width
    gl.canvas.height = height
    gl.viewport(0, 0, width, height)
    rusty.screen = { width, height };
  }

  const render = () => {
    rusty.update()

    gl.clearColor(0.1, 0.1, 0.1, 1)
    gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT)
    gl.enable(gl.DEPTH_TEST)

    update(viewMatrix, idx, data, col)
    // gl.drawArrays(gl.TRIANGLES, 0, 100)
    // gl.drawArrays(gl.LINE_STRIP, 0, 100)
    // gl.drawElements(gl.POINTS, 100, gl.UNSIGNED_SHORT, 0)
    // gl.drawElements(gl.LINE_LOOP, 3, gl.UNSIGNED_SHORT, 0)
    gl.drawElements(gl.TRIANGLES, 11808, gl.UNSIGNED_SHORT, 0)

    requestAnimationFrame(render)
  }

  let dragging = false
  canvas.onmousedown = (e) => { dragging = true }
  canvas.onmouseup = () => { dragging = false }
  canvas.onmousemove = e => {
    if (!dragging) return
    rusty.rotateCamera(e.movementX)
  }

  window.onresize = onresize
  onresize()
  render()
}

main();

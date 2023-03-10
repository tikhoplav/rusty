import { loadModule } from "./rusty";
import { makeShader } from "./webGl";

const canvas = document.createElement('canvas')
document.querySelector('body').append(canvas)
const gl = canvas.getContext("webgl2");

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
  gl.bufferData(gl.ARRAY_BUFFER, 16 * 1024, gl.DYNAMIC_DRAW)

  const vao = gl.createVertexArray()
  gl.bindVertexArray(vao)

  // Setup position attribute, should read from buffer
  // as pairs of float coordinates. Total bytes per vertex is 12.
  gl.vertexAttribPointer(0, 3, gl.FLOAT, false, 16, 0)
  gl.enableVertexAttribArray(0)

  // Setup color attribute, should read from buffer1024 as a vector
  // of 4 bytes. Total bytes per vertex is 12.
  gl.vertexAttribPointer(1, 4, gl.UNSIGNED_BYTE, true, 16, 12)
  gl.enableVertexAttribArray(1)

  function update(buffer: ArrayBuffer, viewMatrix: Float32Array): void {
    gl.useProgram(program)
    gl.uniformMatrix4fv(gl.getUniformLocation(program, "uMatrix"), false, viewMatrix)
    gl.bindBuffer(gl.ARRAY_BUFFER, vertexBuffer)
    gl.bindVertexArray(vao)
    gl.bufferSubData(gl.ARRAY_BUFFER, 0, buffer)
  }

  return { update }
}

loadModule("rusty.wasm")
  .then(rusty => {

    rusty.initState()
    const viewMatrix = rusty.viewMatrix;
    const data = rusty.verticesData;

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

      update(data, viewMatrix)
      gl.drawArrays(gl.TRIANGLES, 0, 6)

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
  })

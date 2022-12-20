import { loadModule } from "./rusty";
import { makeShader } from "./webGl";

const canvas = document.createElement('canvas')
document.querySelector('body').append(canvas)
const gl = canvas.getContext("webgl2");

const spriteRenderer = function (gl: WebGL2RenderingContext) {
  const vertexShaderSource = `#version 300 es
uniform vec2 uResolution;

layout (location = 0) in vec3 aPosition;
layout (location = 1) in vec4 AColor;

out vec4 vColor;

void main() {
  gl_Position = vec4(aPosition, 1) * 2.0 / vec4(uResolution, 1, 1);
  vColor = AColor;
}`

  const fragmentShaderSource = `#version 300 es
precision highp float;

in vec4 vColor;
out vec4 diffuseColor;

void main() {
  diffuseColor = vColor;
}`

  const program = makeShader(gl, vertexShaderSource, fragmentShaderSource)
  gl.useProgram(program)

  const vertexBuffer = gl.createBuffer()
  gl.bindBuffer(gl.ARRAY_BUFFER, vertexBuffer)
  gl.bufferData(gl.ARRAY_BUFFER, 60, gl.DYNAMIC_DRAW)

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

  function resize(width: number, height: number): void {    
    gl.useProgram(program)
    gl.viewport(0, 0, width, height)
    gl.uniform2f(gl.getUniformLocation(program, "uResolution"), width, height)
  }

  function update(buffer: ArrayBuffer): void {
    gl.useProgram(program)
    gl.bindBuffer(gl.ARRAY_BUFFER, vertexBuffer)
    gl.bindVertexArray(vao)
    gl.bufferSubData(gl.ARRAY_BUFFER, 0, buffer)
  }

  return { update, resize }
}

loadModule("rusty.wasm")
  .then(rusty => {

    rusty.initState()
    const data = rusty.getVerticesData();

    const { resize, update } = spriteRenderer(gl)

    const onresize = () => {
      const { innerWidth: width, innerHeight: height} = window
      gl.canvas.width = width
      gl.canvas.height = height
      resize(width, height)
    }

    const render = () => {
      rusty.update()

      gl.clearColor(0.1, 0.1, 0.1, 1)
      gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT)
      gl.enable(gl.DEPTH_TEST)

      update(data)
      gl.drawArrays(gl.TRIANGLES, 0, 3)

      requestAnimationFrame(render)
    }
    
    window.onresize = onresize
    onresize()
    render()
  })

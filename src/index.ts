import { loadModule } from "./rusty";
import { makeShader } from "./webGl";

const canvas = document.createElement('canvas')
document.querySelector('body').append(canvas)
const gl = canvas.getContext("webgl2");

const spriteRenderer = function (gl: WebGL2RenderingContext) {
  const vertexShaderSource = `#version 300 es
uniform vec2 uResolution;

layout (location = 0) in vec2 aPosition;
layout (location = 1) in vec4 AColor;

out vec4 vColor;

void main() {
  vec2 clipSpace = aPosition * 2.0 / uResolution;

  gl_Position = vec4(clipSpace, 0, 1);
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
  gl.bufferData(gl.ARRAY_BUFFER, 36, gl.DYNAMIC_DRAW)

  const vao = gl.createVertexArray()
  gl.bindVertexArray(vao)

  // Setup position attribute, should read from buffer
  // as pairs of float coordinates. Total bytes per vertex is 12.
  gl.vertexAttribPointer(0, 2, gl.FLOAT, false, 12, 0)
  gl.enableVertexAttribArray(0)

  // Setup color attribute, should read from buffer as a vector
  // of 4 bytes. Total bytes per vertex is 12.
  gl.vertexAttribPointer(1, 4, gl.UNSIGNED_BYTE, true, 12, 8)
  gl.enableVertexAttribArray(1)

  function resize(): void {
    const { innerWidth: width, innerHeight: height} = window;
    gl.canvas.width = width;
    gl.canvas.height = height;
    gl.useProgram(program)
    gl.viewport(0, 0, width, height)
    gl.uniform2f(gl.getUniformLocation(program, "uResolution"), width, height)
  }

  function draw(buffer: ArrayBuffer): void {
    gl.clearColor(0.1, 0.1, 0.1, 1)
    gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);
    gl.enable(gl.DEPTH_TEST);

    gl.useProgram(program)
    gl.bindBuffer(gl.ARRAY_BUFFER, vertexBuffer)
    gl.bindVertexArray(vao)
    gl.bufferSubData(gl.ARRAY_BUFFER, 0, buffer)

    gl.drawArrays(gl.TRIANGLES, 0, 3)
  }

  return { draw, resize }
}

loadModule("rusty.wasm")
  .then(rusty => {

    rusty.initState()
    const data = rusty.getVerticesData();

    const { resize, draw } = spriteRenderer(gl);
    window.onresize = resize    
    const render = () => {
      rusty.update()
      draw(data)
      requestAnimationFrame(render);
    }
    
    resize()
    render()
  })

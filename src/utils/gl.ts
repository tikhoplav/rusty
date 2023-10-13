/**
 * Creates a GLSL shader program using provided source codes
 * for vertex and fragment shaders. Throws exception in case
 * if shader can't be compiled.
 */
export function makeShader(
  gl: WebGL2RenderingContext,
  vsSource: string,
  fsSource: string
): WebGLProgram {
  const vs = gl.createShader(gl.VERTEX_SHADER)
  const fs = gl.createShader(gl.FRAGMENT_SHADER)
  const prog = gl.createProgram()
  if (!prog || !vs || !fs) throw new Error("Failed to create program resources")

  gl.shaderSource(vs, vsSource)
  gl.shaderSource(fs, fsSource)
  gl.compileShader(vs)
  gl.compileShader(fs)
  gl.attachShader(prog, vs)
  gl.attachShader(prog, fs)
  gl.linkProgram(prog)

  if (!gl.getProgramParameter(prog, gl.LINK_STATUS)) {
    const msg = `Program link failed:\n${
      gl.getProgramInfoLog(prog)
    }\n${
      gl.getShaderInfoLog(vs)
    }\n${gl.getShaderInfoLog(fs)}`
    gl.deleteProgram(prog)
    throw new Error(msg)
  }

  return prog;
};
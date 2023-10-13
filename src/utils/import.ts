/**
 * Fetches and return content of the binary file as byte array.
 */
export async function loadBinary(src: string): Promise<ArrayBuffer> {
  const res = await fetch(src)
  if (!res.ok) throw new Error(`Fetch failed ${src}: ${res.status} ${res.statusText}`)
  return await res.arrayBuffer()
}
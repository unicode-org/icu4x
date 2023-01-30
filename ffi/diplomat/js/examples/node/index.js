export * from './lib/index.js'

export async function fullData() {
  const path = new URL('./data.postcard', import.meta.url);
  if (typeof fetch === 'undefined') { // Node
    const fs = await import("fs");
    return fs.readFileSync(path);
  } else { // Browser
    const response = await fetch(path);
    return await response.arrayBuffer();
  }
}
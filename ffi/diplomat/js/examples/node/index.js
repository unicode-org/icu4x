export * from './lib/index.js'

export async function fullData() {
    const path = './lib/full.postcard';
    if (typeof fetch === 'undefined') { // Node
        const fs = await import("fs");
        return fs.readFileSync(path)
      } else { // Browser
        const response = await fetch(path);
        return await response.arrayBuffer()
      }
      
}
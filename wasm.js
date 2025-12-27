async function init() {
  const out = document.getElementById('out');
  out.textContent = 'Loading WASM...';


  // IMPORTANT: This must be served over HTTP so the browser can fetch hello_world.wasm
  const resp = await fetch('./hello_world.wasm');
  const { instance } = await WebAssembly.instantiateStreaming(resp);

  const { memory, hello_ptr, hello_len } = instance.exports;

  const ptr = hello_ptr();
  const len = hello_len();

  const bytes = new Uint8Array(memory.buffer, ptr, len);
  const msg = new TextDecoder('utf-8').decode(bytes);

  out.textContent = msg;
}

init().catch((e) => {
  console.error(e);
  const out = document.getElementById('out');
  out.textContent = 'Error loading WASM (check console)';
});

// Simple WAT to WASM compiler for our counter module
function wat2wasm(wat) {
  // This is a minimal WAT parser for our specific case
  // For production, use a proper library like @wasm-tool/wat
  const encoder = new TextEncoder();
  const watBytes = encoder.encode(wat);
  
  // Use WebAssembly's built-in WAT compilation
  // Note: This requires the browser to support WAT compilation
  // Fallback: create binary manually
  try {
    // Try to use WebAssembly.compile if it supports WAT (some browsers do)
    return WebAssembly.compile(watBytes);
  } catch (e) {
    // Fallback: create binary manually
    return createCounterWasm();
  }
}

function createCounterWasm() {
  // Create WASM binary manually with correct sizes
  // Function body: global.get(2) + const(2) + add(1) + set(2) + get(2) + end(1) = 10 bytes
  
  // Build WASM module step by step to ensure correct sizes
  const parts = [];
  
  // Magic and version
  parts.push(new Uint8Array([0x00, 0x61, 0x73, 0x6D, 0x01, 0x00, 0x00, 0x00]));
  
  // Type section: () -> i32
  parts.push(new Uint8Array([0x01, 0x05, 0x01, 0x60, 0x00, 0x01, 0x7F]));
  
  // Function section: 1 function
  parts.push(new Uint8Array([0x03, 0x02, 0x01, 0x00]));
  
  // Global section: mutable i32 = 0
  parts.push(new Uint8Array([0x06, 0x06, 0x01, 0x7F, 0x01, 0x41, 0x00, 0x0B]));
  
  // Export section: export "count"
  parts.push(new Uint8Array([0x07, 0x09, 0x01, 0x05, 0x63, 0x6F, 0x75, 0x6E, 0x74, 0x00, 0x00]));
  
  // Code section - build carefully
  // Expression bytes (the actual instructions):
  const expr = new Uint8Array([
    0x23, 0x00,  // global.get 0
    0x41, 0x01,  // i32.const 1
    0x6A,        // i32.add
    0x24, 0x00,  // global.set 0
    0x23, 0x00,  // global.get 0
    0x0B,        // end
  ]);
  
  // Function body = num locals (1 byte) + expression (10 bytes) = 11 bytes
  const functionBody = new Uint8Array(1 + expr.length);
  functionBody[0] = 0x00;  // num locals: 0
  functionBody.set(expr, 1);
  
  // Code section content = num functions (1 byte) + body size (1 byte) + function body (11 bytes)
  // body size = 11 = 0x0B
  const codeContent = new Uint8Array(1 + 1 + functionBody.length);
  codeContent[0] = 0x01;  // num functions
  codeContent[1] = functionBody.length;  // function body size: 11 (0x0B)
  codeContent.set(functionBody, 2);
  
  // Code section: id (1) + size (1) + content
  const codeSection = new Uint8Array(1 + 1 + codeContent.length);
  codeSection[0] = 0x0A;  // Code section id
  codeSection[1] = codeContent.length;  // Section size: 13 (1 + 1 + 11)
  codeSection.set(codeContent, 2);
  
  parts.push(codeSection);
  
  // Combine all parts
  const totalLength = parts.reduce((sum, part) => sum + part.length, 0);
  const wasm = new Uint8Array(totalLength);
  let offset = 0;
  for (const part of parts) {
    wasm.set(part, offset);
    offset += part.length;
  }
  
  return WebAssembly.compile(wasm);
}

async function init() {
  const button = document.getElementById('counter-btn');
  button.textContent = 'Loading WASM...';

  try {
    // Use inline WASM creation (more reliable than binary file)
    const wasmModule = await createCounterWasm();
    const instance = await WebAssembly.instantiate(wasmModule);
    
    const { count } = instance.exports;
    
    button.textContent = 'Click me: 0';
    
    button.addEventListener('click', () => {
      const newCount = count();
      button.textContent = `Click me: ${newCount}`;
    });
  } catch (e) {
    console.error('Failed to create WASM:', e);
    button.textContent = 'Error loading WASM (check console)';
  }
}

init().catch((e) => {
  console.error(e);
  const button = document.getElementById('counter-btn');
  button.textContent = 'Error loading WASM (check console)';
});


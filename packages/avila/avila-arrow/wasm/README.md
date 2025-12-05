# Avila Arrow WASM

WebAssembly bindings for avila-arrow.

## Build

```bash
wasm-pack build --target web
```

## Usage

```javascript
import init, { WasmArray } from './pkg/avila_arrow_wasm.js';

await init();
const array = new WasmArray();
```

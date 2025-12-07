# Avila Browser - JavaScript Bindings

WebAssembly bindings for using Avila Browser in JavaScript/TypeScript.

## Installation

```bash
npm install avila-browser
```

## Usage

```typescript
import { Browser, SecurityLayer } from 'avila-browser';

// Create browser instance
const browser = new Browser();

// Enable security layers
await browser.enableLayer(SecurityLayer.TOR);
await browser.enableLayer(SecurityLayer.VPN);

// Navigate to URL
await browser.navigate('https://example.com');

// Take screenshot
const screenshot = await browser.screenshot();

// Execute JavaScript
const result = await browser.executeScript('return document.title;');
console.log(result);

// Get cookies
const cookies = await browser.getCookies();
cookies.forEach(cookie => {
    console.log(`${cookie.name}: ${cookie.value}`);
});
```

## Building from Source

```bash
# Install wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Build WASM module
wasm-pack build --target web

# Build npm package
npm run build
```

## API Reference

TODO: Add complete API documentation

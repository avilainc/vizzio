# Avila Browser - Python Bindings

Python bindings for the Avila Browser library using PyO3.

## Installation

```bash
pip install avila-browser
```

## Usage

```python
from avila_browser import Browser, SecurityLayer

# Create browser instance
browser = Browser()

# Enable security layers
browser.enable_layer(SecurityLayer.TOR)
browser.enable_layer(SecurityLayer.VPN)

# Navigate to URL
browser.navigate("https://example.com")

# Take screenshot
browser.screenshot("output.png")

# Execute JavaScript
result = browser.execute_script("return document.title;")
print(result)

# Get cookies
cookies = browser.get_cookies()
for cookie in cookies:
    print(f"{cookie.name}: {cookie.value}")
```

## Building from Source

```bash
# Install maturin
pip install maturin

# Build and install
maturin develop
```

## API Reference

TODO: Add complete API documentation

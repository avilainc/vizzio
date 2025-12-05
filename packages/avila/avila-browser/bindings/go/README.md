# Avila Browser - Go Bindings

Go bindings for the Avila Browser library using CGO.

## Installation

```bash
go get github.com/vizzio/avila-browser/bindings/go
```

## Usage

```go
package main

import (
    "fmt"
    avila "github.com/vizzio/avila-browser/bindings/go"
)

func main() {
    // Create browser instance
    browser := avila.NewBrowser()
    defer browser.Close()

    // Enable security layers
    browser.EnableLayer(avila.LayerTor)
    browser.EnableLayer(avila.LayerVPN)

    // Navigate to URL
    err := browser.Navigate("https://example.com")
    if err != nil {
        panic(err)
    }

    // Take screenshot
    err = browser.Screenshot("output.png")
    if err != nil {
        panic(err)
    }

    // Execute JavaScript
    result, err := browser.ExecuteScript("return document.title;")
    if err != nil {
        panic(err)
    }
    fmt.Println(result)

    // Get cookies
    cookies, err := browser.GetCookies()
    if err != nil {
        panic(err)
    }
    for _, cookie := range cookies {
        fmt.Printf("%s: %s\n", cookie.Name, cookie.Value)
    }
}
```

## Building from Source

```bash
# Build the library
cargo build --release

# Build Go bindings
cd bindings/go
go build
```

## API Reference

TODO: Add complete API documentation

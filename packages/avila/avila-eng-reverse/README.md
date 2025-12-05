# 🔬 Deriax - Advanced Reverse Engineering Tool

<div align="center">

![Rust](https://img.shields.io/badge/rust-1.75+-orange.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20Linux%20%7C%20macOS-lightgrey.svg)

**"Derivar até o último exponente"** - Complete toolkit for binary analysis, malware detection, and reverse engineering

[Features](#features) • [Installation](#installation) • [Usage](#usage) • [Documentation](#documentation) • [Contributing](#contributing)

</div>

---

## 🎯 Overview

Deriax is a comprehensive reverse engineering framework built in Rust, combining static analysis, dynamic analysis, machine learning, and threat intelligence to provide deep insights into binary files. Whether you're analyzing malware, finding vulnerabilities, or solving CTF challenges, Deriax has you covered.

## ✨ Features

### 🔍 Analysis Capabilities
- **Static Analysis**: CFG construction, data flow analysis, symbolic execution, deobfuscation
- **Dynamic Analysis**: Sandbox execution, API hooking, behavior monitoring
- **Code Emulation**: Unicorn-based emulation for x86/x64/ARM architectures
- **Multi-format Support**: PE, ELF, Mach-O, DEX, WASM, .NET, Java, Python bytecode

### 🦠 Malware Detection
- **Signature-based**: YARA rules, fuzzy hashing, import hashing
- **ML-based**: Random Forest and Neural Network classifiers
- **Behavioral**: Pattern matching, anomaly detection
- **Threat Intel**: VirusTotal, AlienVault OTX, MISP integration

### 🛡️ Vulnerability Analysis
- **Memory safety**: Buffer overflows, use-after-free, double-free
- **Injection flaws**: Command injection, SQL injection, format strings
- **Crypto weaknesses**: Weak algorithms, hardcoded keys
- **Logic bugs**: Integer overflows, race conditions

### 🎮 CTF Tools
- **Crypto**: Classical ciphers, RSA attacks, hash analysis
- **PWN**: ROP chain generation, exploit templates, shellcode analysis
- **Forensics**: File carving, metadata extraction, steganography

### 🎨 User Interfaces
- **CLI**: Powerful command-line interface
- **TUI**: Interactive terminal UI with hex viewer, CFG visualization
- **Web API**: REST API for remote analysis
- **Reports**: JSON, HTML, PDF, Markdown export

### 🔧 Advanced Features
- **Plugin System**: Extensible architecture for custom analyzers
- **Caching**: Intelligent caching (memory/disk/Redis) for performance
- **Parallel Processing**: Multi-threaded analysis for speed
- **Cloud Ready**: Docker support, distributed scanning

## 📦 Installation

### From Source

```bash
# Clone repository
git clone https://github.com/username/deriax.git
cd deriax

# Build release
cargo build --release

# Install
cargo install --path .
```

### Using Docker

```bash
docker pull deriax/deriax:latest
docker run -v $(pwd)/samples:/data/input deriax analyze /data/input/malware.exe
```

### Pre-built Binaries

Download from [Releases](https://github.com/username/deriax/releases)

## 🚀 Quick Start

### Basic Analysis

```bash
# Analyze a binary
deriax analyze malware.exe

# Deep scan with all features
deriax analyze --profile paranoid malware.exe

# Generate HTML report
deriax analyze malware.exe --report html --output report.html
```

### Malware Detection

```bash
# Scan with YARA rules
deriax scan --yara-rules ./rules/ malware.exe

# ML-based detection
deriax detect --ml-model ./models/classifier.onnx malware.exe

# Query threat intelligence
deriax threat-intel --hash <sha256> malware.exe
```

### Vulnerability Scanning

```bash
# Scan for vulnerabilities
deriax vuln scan binary.exe

# Find ROP gadgets
deriax vuln rop binary.exe
```

### CTF Tools

```bash
# Decrypt string
deriax ctf decrypt --method xor --key 0x42 encrypted.txt

# Analyze shellcode
deriax ctf shellcode analyze payload.bin

# Frequency analysis
deriax ctf crypto frequency ciphertext.txt
```

### TUI Mode

```bash
# Launch interactive TUI
deriax tui malware.exe
```

### Web Interface

```bash
# Start web server
deriax serve --host 0.0.0.0 --port 8080

# Access at http://localhost:8080
```

## 📖 Documentation

- [User Guide](docs/USER_GUIDE.md)
- [Developer Guide](DEVELOPMENT.md)
- [API Documentation](docs/API.md)
- [Plugin Development](plugins/README.md)
- [Configuration](docs/CONFIGURATION.md)

## 🏗️ Architecture

```
deriax/
├── src/
│   ├── plugin/          # Plugin system
│   ├── cache/           # Caching layer
│   ├── analysis/        # Static & dynamic analysis
│   ├── emulation/       # Code emulation
│   ├── ml/              # Machine learning
│   ├── threat_intel/    # Threat intelligence
│   ├── formats/         # File format parsers
│   ├── malware/         # Malware detection
│   ├── vuln/            # Vulnerability analysis
│   ├── ctf/             # CTF tools
│   ├── tui/             # Terminal UI
│   ├── web/             # Web interface
│   └── reporting/       # Report generation
├── tests/               # Test suite
├── benches/             # Benchmarks
├── plugins/             # Custom plugins
├── rules/               # YARA rules
├── models/              # ML models
└── config.toml          # Configuration

```

## 🤝 Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📊 Roadmap

See [BLUEPRINT.md](BLUEPRINT.md) for detailed roadmap and future features.

**Phase 1** (Q1 2026): Core improvements, dynamic analysis, plugin system
**Phase 2** (Q2 2026): Advanced analysis, ML integration, emulation
**Phase 3** (Q3 2026): Professional features, TUI, reporting
**Phase 4** (Q4 2026): Performance optimization, web UI, cloud integration

## 📄 License

This project is licensed under the MIT License - see [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [Capstone](https://www.capstone-engine.org/) - Disassembly framework
- [Unicorn](https://www.unicorn-engine.org/) - CPU emulator
- [YARA](https://virustotal.github.io/yara/) - Pattern matching
- [VirusTotal](https://www.virustotal.com/) - Threat intelligence
- All contributors and supporters

## 📞 Contact

- **Issues**: [GitHub Issues](https://github.com/username/deriax/issues)
- **Discussions**: [GitHub Discussions](https://github.com/username/deriax/discussions)
- **Email**: deriax@example.com

---

<div align="center">
Made with ❤️ by the Deriax Team | <b>"Derivar até o último exponente"</b> 🔬
</div>

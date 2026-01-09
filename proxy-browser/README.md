# Proxy-Desktop-Browser

A privacy-focused desktop browser built with Tauri, Rust, and Svelte that provides advanced proxy management, fingerprint protection, and tab isolation.

## 🚀 Features

### Privacy & Security
- **Proxy Management**: Support for HTTP, HTTPS, SOCKS4, and SOCKS5 proxies
- **Smart Proxy Rotation**: 10+ rotation strategies including per-request, geographic, and performance-based
- **Fingerprint Protection**: Canvas, WebGL, audio, and navigator spoofing
- **Tab Isolation**: Each tab can have its own proxy and identity
- **Ad & Tracker Blocking**: Built-in content filtering

### Developer Experience
- **Modern Stack**: Tauri 2.0 + Rust + Svelte 5 + TypeScript
- **Type Safety**: Full TypeScript coverage with strict mode
- **Structured Logging**: Comprehensive logging system with levels
- **Modular Architecture**: Clean separation of concerns

### Performance
- **Efficient Resource Usage**: Native performance with Rust backend
- **Connection Pooling**: Optimized network connections
- **Lazy Loading**: Components loaded on demand
- **Memory Management**: Automatic cleanup and optimization

## 📦 Tech Stack

| Layer | Technology |
|-------|------------|
| Frontend | Svelte 5, TypeScript, TailwindCSS |
| Desktop Runtime | Tauri 2.0 |
| Backend | Rust |
| Browser Engine | Chromium (via chromiumoxide) |
| Build Tool | Cargo, Bun/npm |

## 🛠️ Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (1.70+)
- [Node.js](https://nodejs.org/) (18+) or [Bun](https://bun.sh/)
- [Tauri CLI](https://tauri.app/v1/guides/getting-started/prerequisites)

### Installation

1. Clone the repository
2. Install frontend dependencies: `cd ui-tauri && npm install`
3. Build the application: `cargo build --release`
4. Run in development mode: `cargo tauri dev`

### Using the Makefile

- `make build` - Build everything
- `make test` - Run tests
- `make clean` - Clean build artifacts
- `make fmt` - Format code
- `make lint` - Run linter

## 📁 Project Structure

- `crates/browser-core/` - Core browser functionality (Rust)
- `ui-tauri/src/` - Frontend components (Svelte/TypeScript)
- `ui-tauri/src-tauri/` - Tauri backend (Rust)
- `config/` - Configuration files

## 📚 Documentation

- [Architecture Diagrams](ARCHITECTURE_DIAGRAMS.md)
- [Codebase Understanding](CODEBASE_UNDERSTANDING.md)
- [Proxy System Documentation](PROXY_CODEBASE_UNDERSTANDING.md)
- [Development Phases](DEVELOPMENT_PHASES.md)
- [Getting Started Guide](GETTING_STARTED.md)
- [Improvements Log](IMPROVEMENTS.md)
- [Changelog](CHANGELOG.md)

## 🧪 Testing

- Run all tests: `cargo test`
- Run with output: `cargo test -- --nocapture`
- Run frontend tests: `cd ui-tauri && npm test`

## 🤝 Contributing

1. Fork the repository
2. Create your feature branch
3. Commit your changes
4. Push to the branch
5. Open a Pull Request

### Code Quality Standards

- Run `cargo fmt` before committing
- Ensure `cargo clippy` passes without warnings
- Add tests for new functionality
- Update documentation as needed

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

**Note**: This project is under active development. See [CHANGELOG.md](CHANGELOG.md) for recent updates.


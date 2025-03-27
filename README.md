# Standard PyOxidizer

A Rust-Python hybrid project demonstrating interoperability between Python and Rust using PyO3 and PyOxidizer.

## Features

- Python extensions written in Rust
- Rust performance with Python convenience
- Cross-language type conversions
- Comprehensive test suite

## Installation

### Prerequisites
- Rust (install via [rustup](https://rustup.rs/))
- Python 3.7+
- PyOxidizer (`pip install pyoxidizer`)

### Build Instructions
```bash
# Clone the repository
git clone https://github.com/KayanoLiam/Standard_PyOxidizer.git
cd Standard_PyOxidizer

# Build the project
pyoxidizer build

# Install Python package
pip install -e .
```

## Project Structure

```
Standard_PyOxidizer/
├── src/                  # Rust source files
│   ├── lib.rs            # Main Rust library
│   ├── abs.rs            # Absolute value implementations
│   ├── ascii.rs          # ASCII utilities
│   └── ...               # Other Rust modules
├── scripts/              # Python scripts and tests
│   ├── abs_test.py       # Absolute value tests
│   ├── classtest.py      # Class testing
│   └── Standard_PyOxidizer.pyi  # Python interface
├── Cargo.toml           # Rust package configuration
├── pyproject.toml       # Python package configuration
└── .github/workflows/   # CI/CD pipelines
```

## Usage

Import and use the Rust extensions in Python:

```python
from Standard_PyOxidizer import absolute_value

print(absolute_value(-42))  # 42
```

## Development

Run tests:
```bash
cargo test
python -m pytest scripts/
```

## License

MIT License - See [LICENSE](LICENSE) for details.

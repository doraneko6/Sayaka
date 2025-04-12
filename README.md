# Sayaka

Sayaka is a lightweight Rust debugging toolkit that provides concise and practical macros to assist in development debugging.

## Features

- Colored formatted debug output
- Configurable debug information display
- Easy-to-use macros for printing variables and function call information

## Installation

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
sayaka = "0.2.0"
```

or use the cargo command in you commandline.

```
cargo add sayaka
```

## Usage

### Basic Usage

```rust
use sayaka::{debugln, debug_var, debug_fn};

fn main() {
    let name = "world";
    
    // Print simple debug information
    debugln!("Hello, {}!", name);
    
    // Print variable value and type
    debug_var!(name);
    
    // Track function calls and their parameters
    my_function(42);
}

fn my_function(value: i32) {
    debug_fn!(value);
    // Function body...
}
```

### Environment Variables

Sayaka can be configured through two environment variables:

- `SAYAKA_DEBUG`: Set to a non-zero value to enable debug output
- `SAYAKA_NO_COLOR`: Set to a non-zero value to disable colored output

Examples:

```sh
# Enable debug output
SAYAKA_DEBUG=1 cargo run

# Enable debug but disable colored output
SAYAKA_DEBUG=1 SAYAKA_NO_COLOR=1 cargo run
```

## API Reference

### Macros

- `debugln!($fmt, ...)`: Prints debug information, similar to `println!` format
- `debug_var!($var1, $var2, ...)`: Prints detailed information about one or more variables
- `debug_fn!($arg1, $arg2, ...)`: Prints current function name and its parameters

## License

MIT
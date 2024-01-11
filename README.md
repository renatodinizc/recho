# Rust ECHO Utility

`recho` is a simple command-line utility written in Rust that echoes input arguments with optional newline control. It is designed to provide flexibility in printing text, allowing users to control whether a newline character should be appended to the output or not.

## Usage

To use Recho, build the project and run the executable, passing the desired input arguments:

```bash
cargo build --release
./target/release/recho [OPTIONS] [INPUTS...]
```

### Options

- `-n`: Do not print a newline character at the end of the output.

### Examples

1. Echo with a newline:

```bash
./target/release/recho Hello
# Output: Hello
```

2. Echo without a newline:

```bash
./target/release/recho -n Hello World
# Output: Hello World%
```

3. Combining options and inputs:

```bash
./target/release/recho Apple pie -n recipe
# Output: Apple pie -n recipe
```

## Tests

To run the test suit, use the following command:

```bash
cargo test
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE.md) file for details.
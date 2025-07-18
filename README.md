# image_to_ascii

A simple Rust command-line tool that converts an image into ASCII art and prints it to the terminal.

## Features

- Reads an image file (e.g., PNG, JPEG).
- Converts the image pixels to ASCII characters based on brightness.
- Prints the ASCII art to the terminal.
- (Optional) Can write the ASCII art to a file (`hello.txt`).

## Usage

```sh
cargo run -- path/to/your/image.png
```

- Replace `path/to/your/image.png` with the path to your image file.

## Example

If you have an image at `photos/dinosaur.png`:

```sh
cargo run -- photos/dinosaur.png
```

## How it works

- The program reads the image and converts each pixel’s brightness to an ASCII character:
  - Darkest: `a`
  - Medium-dark: `_`
  - Medium-bright: `$`
  - Brightest: `D`
- The ASCII art is printed line by line to the terminal.

## Requirements

- Rust (install from [rustup.rs](https://rustup.rs))
- The `image` crate (handled by Cargo)

## License

MIT

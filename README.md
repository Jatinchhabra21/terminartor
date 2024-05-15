### Terminartor

Terminartor is a command line tool designed to generate terminal art from images. It converts images into ASCII art that can be displayed directly in the terminal. The tool requires two mandatory options: -p or --path to specify the image path (absolute), and -s or --scale to scale the image before converting it to ASCII art.

### Installation

- Download the binary file from release page.
- Add directory in which binary is present to $PATH variable
  OR
- Move binary to `/usr/local/bin` (for linux)

### Usage

Terminartor requires two options: -p or --path to specify the image file path and -s or --scale to define the scaling factor.

#### Help

```sh
terminartor -h
```

#### Options

- -p, --path: Path to the image file (mandatory).
- -s, --scale: Scaling factor to resize the image before converting to ASCII art (mandatory).

#### Example

```sh
terminartor -p /path/to/img -s 25
```

### Contributing

Contributions are welcome! Please open an issue or submit a pull request for any improvements or bug fixes.

Enjoy creating terminal art with Terminartor!

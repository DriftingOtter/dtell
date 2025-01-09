# dtell (date tell)

> A tool to calculate dates based on a number of days relative to today.

## Installation and Build

Clone the repository and navigate into it:
```bash
git clone https://github.com/DriftingOtter/dtell.git
cd dtell
```

Build the project:
```bash
cargo build --release
```

### Optional: Move Binary to Local Binaries Folder

To access `dtell` without specifying the binary path, move it to your local binaries folder:
```bash
cp target/release/dtell ~/.local/bin
```

Ensure `~/.local/bin` is in your `PATH`. You can add it by updating your shell configuration file (`~/.bashrc` or `~/.zshrc`):
```bash
export PATH="$HOME/.local/bin:$PATH"
```

## Synopsis

- **Basic usage**: `dtell <number of days>`
- **Days option**: `dtell --days <number>`, `dtell -d <number>`
- **Format option**: `dtell --format <type>`, `dtell -f <type>`

- **Help mode**: `dtell -h` or `dtell --help`

### Additional Arguments

- `--days, -d <number>`: Specify the number of days to add or subtract relative to today.
- `--format, -f <type>`: Specify the output format for the date. Supported types are:
  - `gen`: Date displayed in DD/MM/YYYY format.
  - `us`: Date displayed in MM/DD/YYYY format.
  - `iso`: Date displayed in ISO format (YYYY-MM-DD). This is the default format if no format is specified.
- `--help, -h`: Display help information for the command.

## Examples

- `dtell --d 5 --f iso`: Displays the date 5 days in the future in ISO format.
- `dtell 5`: Displays the date 5 days in the future in ISO format (default).
- `dtell --days -10 --format us`: Displays the date 10 days in the past in US format (MM/DD/YYYY).
- `dtell -d cat`: Displays an error message, as 'cat' is not a valid integer.
- `dtell --days 7`: Displays the date 7 days in the future in ISO format (default).

## Authors

- Daksh Kaul // DriftingOtter 🦦


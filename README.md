# vatax

Useless vat calculator. Do not use, use a calc please. I'm just learning rust.

## Usage
```
raph:vatax raphael$ ./target/debug/vatax --help
Vat calculator.

Usage:
  vatax [-r] <value> [--rate=<rate>]
  vatax (-h | --help)
  vatax --version

Options:
  -r                  Compute from value without tax to value with tax.
  --rate=<rate>       Change vat rate [default: 20]
  --version           Get version
  -h --help           Show this screen.
  ```
  
## Example
```bash
raph:vatax raphael$ ./vatax 42
Without tax   42.00
With tax      50.40

raph:vatax raphael$ ./vatax -r 42
Without tax   35.00
With tax      42.00
```

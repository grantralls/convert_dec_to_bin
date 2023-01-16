# CS417 Machine Assignment 1 Converting Decimal Mantissa to Binary Mantissa

## Requirements

-   cargo v1.66.0 or newer

## Compilation

1. Clone the repository
2. Run `cargo build --release` in the root directory of the repository
3. The executable will be located in `target/release/` named `convert_dec_to_bin`

## Usage

1. Complete the steps in the compilation section and navigate to the `target/release/` directory
2. Run `./convert_dec_to_bin <decimal float> <decimal float>...`
3. The output will be the binary representations of the decimal floats in a table

### Example

```bash
$ ./convert_dec_to_bin 0.625 .1 .5
|Base 10    | Base 2    |
|0.625      | 0.101     |
|0.1        | 0.00011001|
|0.5        | 0.1       |
```

## Testing

1. Navigate to the root directory of the repository
2. Run `cargo test`

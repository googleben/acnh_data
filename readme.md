# acnh_data

This project is for converting ACNH's .bcsv and .msbt files into more usable data.

## Use

First, create a directory called `bcsv` in the project directory, fill it with the .bcsv files, and place the enum definitions file named `enums.json` in the project directory. Next, run the code generation step with the command `cargo run --no-default-features --features "codegen"`. Now the project will have definitions for all the .bcsv files in `src/bcsv/defs.rs`, and can be run with `cargo run` for generation of actual data.
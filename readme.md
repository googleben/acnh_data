# acnh_data

This project is for converting ACNH's .bcsv and .msbt files into more usable data.

## Use

- Create a directory called `input` in the project directory, and copy the extracted `Bcsv` and `Messages` folders from ACNH's ROMFS
- Place the enum definitions file named `enums.json` in the project directory. 
- Run the code generation step with the command `cargo run --no-default-features --features "codegen"`

The project will now have definitions for all the .bcsv files in `src/bcsv/defs.rs`, and can be run with `cargo run` for generation of actual data.
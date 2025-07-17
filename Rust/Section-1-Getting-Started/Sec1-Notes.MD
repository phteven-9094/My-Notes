# Getting Started

## Intro to Rust

- Rust is a systems programming language
- Systems programming refers a category of languages that are optimized for situations where resources (memory and CPU) are limited.
- What can Rust build?
  - CLIs
  - Build Tools
  - Device Drivers
  - Databases
  - Programming languages
  - Web applications
  - Operating Systems
- The Rust Compiler
  - The compiler is a program that translates our source code into an executable program
  - The executable is called a binary or a binary executable
  - The computer can understand and execute the instructions in the binary
- Rustup

  - A utility tool that runs from the CLI that helps with various rust related functionality such as updates and uninstalling

- Create Rust Project with Cargo

  - Cargo is a command-line tool that helps manage Rust projects
  - To create a new project:
    - cargo new <project_name>
    - This will create a new package containing a src directory and a Cargo.toml file
    - By default, this creates a standalone program or a binary(application/crate)
    - The toml file contains metadata and dependencies for the project

- Hello World

  - The function keyword is fn
  - Then it will be named after
  - Every rust file requires a main function
  - A full function example:

    ```Rust
    fn main() {

    }
    ```

  - Each command requires a semi-colon to denote the end of a command
  - See hello-world/hello_world/src/main.rs

- Compiling and Running Rust file from the Terminal

  - rustc file.rs -- this will let the compiler to read and compile the file
  - ./ file -- Indicates the current directory and the file that got created by the compiler

- Formatting with rustfmt and cargo fmt

  - rustfmt file.rs -- Just formats the file to be cleaner
  - cargo fmt -- Run at the top-level of the directory and it will format all files within the project

- The cargo build command

  - cargo build -- compiles all files in a directory
  - ./target/debug/project -- runs the project in debug mode
  - cargo build --release
    - Build the final executable file that is optimized for file size
  - cargo clean
    - Will clean up the directory and restart it from scratch
    - This is so it will not use anything that might be cached for a clean build

- The cargo run command

  - Achieves both compilation and exwcution
  - cargo run -- Builds the executable in debug mode and runs it
  - cargo run --quiet
    - This will just return the output and not all the details

- The cargo check command

  - Checks the source code for compiler violations but does not create an executable

- Comments
  - Indicated by 2 forward slashes //
  - For large comment block /\* \*/

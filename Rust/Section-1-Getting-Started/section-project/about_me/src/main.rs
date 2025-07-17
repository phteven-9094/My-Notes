/*
Create a new `about_me` project with the `cargo new` command. √

Using the `println!` macro, output 3 sentences about yourself.
Feel free to invoke the macro multiple times. √

From the Terminal, compile the `main.rs` file inside the `src`
folder with the Rust compiler, then manually run the executable. √

- Steps taken:
    - Navigate to the project directory.
    - Use `rustc src/main.rs` to compile the file.
    - Run the executable with `./main`.

From the Terminal, compile the project with the Cargo tool, then
manually run the executable. √
- Steps Taken:
    - Navigate to the project directory.
    - Use `cargo build` to compile the project.
    - Run the executable with `./target/debug/about_me`.

From the Terminal, compile and run the project with a single
Cargo command. √

-Steps Taken:
    - Navigate to the project directory.
    - Use `cargo run` to compile and run the project.

Check your program for errors with `cargo check`. √

Add a comment at the top of your source code explaining how to
compile the program for new Rust developers. √

Add some spaces and line breaks to the code so that it is formatted
in an ugly manner. From the Terminal, style the code with the
`cargo fmt` command. √

Replace the `println!` macro with `print!`. What happens?
It printed all of the lines out on the same line with no spaces between
*/

fn main() {
    // To compile this program, use the command:
    // `cargo run` from the project directory.
    println!("I am 35");
    println!("I am a software developer");
    println!("I love learning new programming languages");
}

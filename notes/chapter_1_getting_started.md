# Chapter 1 Notes: Getting Started

## Basic Usage + `rustc`
- A rust `main` function is the first function that is called when a program is ran. 
- We can print stuff to the standard output using the `println!` or `print!` macros. `println!` adds a new line. (e.g "print line"). 
- The reserved keyword `fn` starts the function definition.
- Rust style is to indent with 4 spaces and not a tab.
- The `!` parameter indicates that we are running macro rather than a function. For example: `println!`.
- To compile a program, we can use the `rustc` command, which stands for rust compiler.
- The executable coming from the `rustc` compiler can be run in any computer that shares the same CPU architecture. The computer that executes the compiled file does not need to have rust installed.

## Cargo
- `cargo` is rust's dependency manager tool. It is the rust requivalent of python's `pip`. Different from `pip`, it is also used to build the packages.
  - By default when you run `cargo new`, it creates a directory of the name you gave with the following:
    - `Cargo.toml`: specifies the project's dependencies;
    - `src/main.rs`: a sample hello world file;
    - Initializes a git repo with a gitignore file as well.
      - To override this, use `cargo new --vcs=none` (no version control system). This is useful if a project is placed in an existing git repo.
- In the `Cargo.toml`, the package definition is described, along with its version and edition. Unclear to me what an edition means.
- In the `[dependencies]` section, we list the `crates` (rust's equivalent to pip packages I suppose) that the project requires.
- `cargo run` builds the executable from the code in the project and runs it right after. It is more common than running `cargo build` first.
- `cargo check` checks the code for compile issues but does not produce an executable.
- `cargo build --release`: an option to build an optimized version of the executable, it takes longer to compile that the regular command (debug version). The resulting file is stored on `target/release` instead of `target/debug`.
  
  
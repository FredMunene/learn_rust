
Coursework for learning Rust for blockchain projects.

+ What is Cargo and why do you need it?
    * Cargo is a *Rust package manager* It downloads your package's dependencies, compiles your packages, makes distributable packages, and uploads them to [crates.io](https://crates.io/).

+ To install Rust and Cargo on a Linux OS, run below command on the terminal.

`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

For other OS, check this [link](https://forge.rust-lang.org/infra/other-installation-methods.html) for instructions

Run the command to store the projects.
` cargo new folder_name` 

A typical setup for a Rust project will contain the below. In our case, folder *folder_name* should contain:

* Cargo.lock
* Cargo.toml
* src
* target

+ Commands
    * `cargo build`
    * `cargp run`
    * `cargo check`
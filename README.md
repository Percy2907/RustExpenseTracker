# Overview

I created an expense tracker that runs in the terminal. The program allows a user to add expenses, display the complete expense list, search expenses by category, calculate the total amount, and delete an expense using its ID. Each expense has an ID, description, category, and amount.

I wrote this software to learn the basic syntax of Rust. I also wanted to practice mutable and immutable variables, expressions, conditionals, loops, functions, ownership, borrowing, references, vectors, structs, and impl blocks in one small project.

[Software Demo Video](https://youtu.be/zW5JuLJjWRg)

# Development Environment

I used Visual Studio Code to write the program and PowerShell to compile and run it. I used Cargo to build, format, and test the program, rust-analyzer to review the Rust syntax, and Git for version control.

The program was written in Rust. It uses the Rust standard library and the `std::io` module to read information from the terminal. It uses a `Vec` to store the expenses while the program is running. No external libraries were required.

# Useful Websites

- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [Rust Standard Library](https://doc.rust-lang.org/std/)
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/)

# Future Work

- Save the expenses in a file.
- Add an option to edit an expense.
- Add dates and monthly expense summaries.

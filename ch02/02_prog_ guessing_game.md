## Programming a Guessing Game
Let’s jump into Rust by working through a hands-on project together! 
This chapter introduces you to a few common Rust concepts by showing 
you how to use them in a real program. You’ll learn about `let, match`, 
methods, associated functions, external crates, and more! 
In the following chapters, we’ll explore these ideas in more detail. 
In this chapter, you’ll just practice the fundamentals.

We’ll implement a classic beginner programming problem: a guessing game. 
Here’s how it works: the program will generate a random integer between 1 and 100.
It will then prompt the player to enter a guess. After a guess is entered, 
the program will indicate whether the guess is too low or too high. 
If the guess is correct, the game will print a congratulatory message and exit.

### Setting Up a New Project

To set up a new project, go to the projects directory that you created in Chapter 1 and make a new project using Cargo, like so:

> $ cargo new guessing_game <br/>
> $ cd guessing_game

The first command, cargo new, takes the name of the project (guessing_game) 
as the first argument. The second command changes to the new project’s directory.

Look at the generated Cargo.toml file:

Filename: **Cargo.toml**

```
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2021"

// See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

### Processing a Guess

The first part of the guessing game program will ask for user input, process that input, and check that the input is in the expected form. To start, we’ll allow the player to input a guess. Enter the code in Listing 2-1 into src/main.rs.

Filename: src/main.rs

#### Project **p01_game_guess**

```rust
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
```
#### Project **p02_game_guess**

```rust
fn main() {
    let x = 5;
    let y = 10;

    println!("x = {x} and y + 2 = {}", y + 2); // Placeholder
}
```

#### Project **p03_game_guess**

```rust
    use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_num = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is = {secret_num}");

    loop {
        println!("Please, input you guess.");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Too win!");
                break;
            }
        }
    }
}
```

### Summary

This project was a hands-on way to introduce you to many new Rust concepts: 
`let, match`, functions, the use of external crates, and more. In the next few 
chapters, you’ll learn about these concepts in more detail. Chapter 3 covers 
concepts that most programming languages have, such as variables, data types, 
and functions, and shows how to use them in Rust. Chapter 4 explores ownership, 
a feature that makes Rust different from other languages. Chapter 5 discusses 
structs and method syntax, and Chapter 6 explains how enums work.
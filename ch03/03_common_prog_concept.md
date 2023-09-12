## Common Programming Concepts

This chapter covers concepts that appear in almost every programming language and how they work in Rust. Many programming languages have much in common at their core. None of the concepts presented in this chapter are unique to Rust, but we’ll discuss them in the context of Rust and explain the conventions around using these concepts.

Specifically, you’ll learn about variables, basic types, functions, comments, and control flow. These foundations will be in every Rust program, and learning them early will give you a strong core to start from.

### Variables and Mutability

#### Declare variable with keyword `let` and `mut`
```rust
fn main() {
    // let x = 5; If we are use this then give us ^^^^^ cannot assign twice to immutable variable
    let mut x = 5;
    println!("x = {}",x);
    println!("x = {x}");
    x = 7;
    println!("x = {x}");
}
```

#### Declare variable with keyword `const`
```rust
fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 3 * 50 * 60;// const is immutable variable and we use const instead of let

    println!("Constant in RUST  const THREE_HOURS_IN_SECONDS: {THREE_HOURS_IN_SECONDS}")
}
```

#### Shadowing
Rustaceans say that the first variable is shadowed by the second, which means that the second variable is what the compiler will see when you use the name of the variable. In effect, the second variable overshadows the first, taking any uses of the variable name to itself until either it itself is shadowed or the scope ends. We can shadow a variable by using the same variable’s name and repeating the use of the let keyword as follows:

```rust
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}")
    }
    println!("The value of x is : {}", x);
}
```

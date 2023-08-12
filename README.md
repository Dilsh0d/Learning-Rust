# Learning-Rust

The Rust Programming Language
by Steve Klabnik and Carol Nichols, with contributions from the Rust Community

https://doc.rust-lang.org/stable/book/
## Introduction
1. [Getting Started](#getting-started)
2. [Common Programming Concepts](#common-programming-concepts)
3. [Understanding Ownership](#understanding-ownership)
4. [Using Structs to Structure Related Data](#using-structs-to-structure-related-data)

## Getting Started
### Installation
The first step is to install Rust. We’ll download Rust through rustup, a command line tool for managing Rust versions and associated tools. You’ll need an internet connection for the download.
#### Installing rustup on Linux or macOS
If you’re using Linux or macOS, open a terminal and enter the following command:

```$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh```
The command downloads a script and starts the installation of the rustup tool, which installs the latest stable version of Rust. You might be prompted for your password. If the install is successful, the following line will appear:

```Rust is installed now. Great!```

You will also need a linker, which is a program that Rust uses to join its compiled outputs into one file. It is likely you already have one. If you get linker errors, you should install a C compiler, which will typically include a linker. A C compiler is also useful because some common Rust packages depend on C code and will need a C compiler.

On macOS, you can get a C compiler by running:

```$ xcode-select --install```
Linux users should generally install GCC or Clang, according to their distribution’s documentation. For example, if you use Ubuntu, you can install the ```build-essential``` package.
#### Installing rustup on Windows
On Windows, go to https://www.rust-lang.org/tools/install and follow the instructions for installing Rust. At some point in the installation, you’ll receive a message explaining that you’ll also need the MSVC build tools for Visual Studio 2013 or later.

To acquire the build tools, you’ll need to install Visual Studio 2022. When asked which workloads to install, include:

  1. “Desktop Development with C++”
  2. The Windows 10 or 11 SDK
  3. The English language pack component, along with any other language pack of your choosing
     
The rest of this book uses commands that work in both cmd.exe and PowerShell. If there are specific differences, we’ll explain which to use.

#### Troubleshooting

To check whether you have Rust installed correctly, open a shell and enter this line:

```$ rustc --version```

You should see the version number, commit hash, and commit date for the latest stable version that has been released, in the following format:

#### Hello world!
Now that you’ve installed Rust, it’s time to write your first Rust program. It’s traditional when learning a new language to write a little program that prints the text Hello, world! to the screen, so we’ll do the same here!

##### Creating a Project Directory

For Linux, macOS, and PowerShell on Windows, enter this:

> $ mkdir ~/projects <br>
> $ cd ~/projects <br>
> $ mkdir hello_world <br>
> $ cd hello_world <br>

For Windows CMD, enter this:

> mkdir "%USERPROFILE%\projects" <br>
> cd /d "%USERPROFILE%\projects" <br>
> mkdir hello_world <br>
> cd hello_world <br>

##### Writing and Running a Rust Program
Now open the main.rs file you just created and enter the code in Listing 1-1.

Filename: main.rs
```rust
fn main() {
    println!("Hello, world!");
}
```
Listing 1-1: A program that prints Hello, world!

Save the file and go back to your terminal window in the ~/projects/hello_world directory. On Linux or macOS, enter the following commands to compile and run the file:

> $ rustc main.rs <br>
> $ ./main <br>

`Hello, world!`

On Windows, enter the command .\main.exe instead of ./main:

> rustc main.rs <br>
> .\main.exe <br>

`Hello, world!`

#### Hello, Cargo!
Cargo is Rust’s build system and package manager. Most Rustaceans use this tool to manage their Rust projects because Cargo handles a lot of tasks for you, such as building your code, downloading the libraries your code depends on, and building those libraries. (We call the libraries that your code needs dependencies.)
Cargo is installed by entering the following in your terminal:

`$ cargo --version`

##### Creating a Project with Cargo

Let’s create a new project using Cargo and look at how it differs from our original “Hello, world!” project. Navigate back to your projects directory (or wherever you decided to store your code). Then, on any operating system, run the following:

> $ cargo new hello_cargo <br>
> $ cd hello_cargo <br>

Go into the hello_cargo directory and list the files. You’ll see that Cargo has generated two files and one directory for us: a Cargo.toml file and a src directory with a main.rs file inside.

Open Cargo.toml in your text editor of choice. It should look similar to the code in Listing 1-2.

Filename: Cargo.toml
```rust
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```
###### Building and Running a Cargo Project
Now let’s look at what’s different when we build and run the “Hello, world!” program with Cargo! From your hello_cargo directory, build your project by entering the following command:

`$ cargo build
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 2.85 secs`

## Common Programming Concepts

## Understanding Ownership

## Using Structs to Structure Related Data

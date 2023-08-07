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

Let’s start your Rust journey! There’s a lot to learn, but every journey starts somewhere. In this chapter, we’ll discuss:

Installing Rust on Linux, macOS, and Windows
Writing a program that prints Hello, world!
Using cargo, Rust’s package manager and build system

Installing rustup on Linux or macOS
If you’re using Linux or macOS, open a terminal and enter the following command:

$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
The command downloads a script and starts the installation of the rustup tool, which installs the latest stable version of Rust. You might be prompted for your password. If the install is successful, the following line will appear:

Rust is installed now. Great!
You will also need a linker, which is a program that Rust uses to join its compiled outputs into one file. It is likely you already have one. If you get linker errors, you should install a C compiler, which will typically include a linker. A C compiler is also useful because some common Rust packages depend on C code and will need a C compiler.

On macOS, you can get a C compiler by running:

$ xcode-select --install
Linux users should generally install GCC or Clang, according to their distribution’s documentation. For example, if you use Ubuntu, you can install the build-essential package.

## Common Programming Concepts

## Understanding Ownership

## Using Structs to Structure Related Data

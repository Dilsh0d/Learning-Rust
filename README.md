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

> mkdir "%USERPROFILE%\projects"
> cd /d "%USERPROFILE%\projects"
> mkdir hello_world
> cd hello_world

## Common Programming Concepts

## Understanding Ownership

## Using Structs to Structure Related Data

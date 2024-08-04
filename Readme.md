# 🐚 0-Shell

This project aims to replicate some core functionalities of the shell program, providing a command-line interface with built-in commands. The project is written in Rust, leveraging its performance and safety features to create a robust shell experience.

## ✨ Features

- Built-in Commands: Implementation of several key commands like `ls`, `cd`, `pwd`, `echo`, `mv`
- Colored Output: Visual distinction of files and directories with colored outputs.

## 🔧 Getting Started

1. Prerequisites
   Rust: Ensure that you have Rust installed. If not, install it from rust-lang.org.
2. Installation

```bash
git clone https://learn.zone01dakar.sn/git/bindoye/0-shell/.git
cd 0-shell
```

3. Build the project

```bash
cargo build --release
```

4. Run the shell

```bash
./target/release/0-shell
```

## 📜 Usage

```bash
ls -l -a -F

cd cd /path/to/directory

pwd

echo "Welcome to the Bash Shell Clone!"
```

## 📦 Packages Used

The project utilizes several Rust crates to facilitate various functionalities:

- `chrono`: For date and time manipulation, particularly in formatting file timestamps.
- `colored`: For adding colored output to distinguish files, directories, and other elements in the shell.
- `libc`: Provides bindings to native C libraries, used here to retrieve user and group information.
- `tabular`: For output formatting

> These crates are included in the Cargo.toml file and will be automatically downloaded and built when you compile the project.

# Tosh
## Toft Shell: My own shell for fun and learning

## Description
A basic shell for Linux, written in Rust.  
Written because it's fun, and because I want to better my understanding of both Rust and the shell.  
Inspired by [Build Your Own Shell using Rust](https://www.joshmcguigan.com/blog/build-your-own-shell-rust/) by Josh Mcguigan.  
Then I started adding more features myself.

## Features
### From the guide mentioned above
* Run commands
* `cd`
* `exit`
* Basic error handling
* Pipes

### Features I've added myself
* Display username
* Display hostname
* Display current directory
* Change displayed location to `~` when under home directory
* `cd` without arguments changes directory to home directory
* Use colors to display prompt
* Save commands to history file

### Features I would like to implement
* Read and use recently used commands from history
* Support for files and folders with spaces in name
* Tab completion
* Suggestions (based on history and maybe even Markov chaining) like in fish-shell

## Requirements
* [Rust](https://www.rust-lang.org/learn/get-started)

## How to run
```
$ cargo run --release
```

## How to exit
```
$ exit
```

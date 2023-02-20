# Mini Project 4

## Description

This is a simple project that converts between a ASCII string and Morse code.

## Steps to run

- `make format` to format code
  
- `make lint` to lint
  
- `cargo run "Hello World"` to run the program
  - Then you will get

```rust

Enter a string to convert:
Go Duke
--. ---   -.. ..- -.- .
Enter Morse code to convert:
--. ---   -.. ..- -.- .
GO  DUKE

```

- `make release-arm` to build for arm which is: `cargo lambda build --release --arm64`
  
- `make deploy` which is this `cargo lambda deploy`

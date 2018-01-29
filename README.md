# What is this project?

This is a Rust crate allowing to communicate with the **HD44780** LCDs.

Currently supports connecting via 4-bit **GPIO** bus or an **I2C** one.

Additionally, there has been provided a buffered version of the driver, allowing for a faster usage in cases when screen
has to be redrawn multiple times in a second (take a look into the `examples` or `src`).

# Would you mind showing me some code?

For examples, take a into the `examples` directory.

# License

```
Copyright (c) 2018, Patryk Wychowaniec <wychowaniec.patryk@gmail.com>.
Licensed under the MIT license.
```
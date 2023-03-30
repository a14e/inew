# INew – a library for generating constructors

In Rust, writing constructors is common but can be repetitive and boring. This library simplifies the process,
making it more enjoyable and freeing up time for more interesting tasks.

The purpose of this library is to cover the most basic and frequent case. If you want more complex generation, you
should probably take a look at  [rust-derive-builder](https://github.com/colin-kiegel/rust-derive-builder)

# How to add the library to your project?

Just add to Cargo.toml

```toml
[dependencies]
inew = "0.2.0"
```

# Мinimum supported Rust version

The library requires a minimum Rust version of `1.56.0` and utilizes this version for executing tests within the CI
environment.

# Example

Suppose you have a structure and constructor, and we want to make a constructor for it.
And it looks like this

```rust
struct MyStruct {
    x: u32,
    y: u16,
    z: String,
    field: String,
    another_field: String
}

impl MyStruct {
    pub fn new(x: u32,
               y: u16,
               z: String,
               field: String,
               another_field: String) -> Self {
        Self {
            x,
            y,
            z,
            field,
            another_field
        }
    }
}
```

But everything here is very obvious, all fields and types are known to compiler. Therefore, we can hand over constructor
generation to a macro

```rust
use inew::New;

#[derive(New)]
struct MyStruct {
    x: u32,
    y: u16,
    z: String,
    field: String,
    another_field: String
}
```

That's it, just add the New annotation

# Default fields and custom functions for generating fields

If you don't want to pass all the fields, you can fill in some of the fields using annotations `#[new(default)]` for
initialization with `Default::default()` or `#[new(default = my_func_name())]` for initialization by calling
my_func_name().
Example of usage

```rust
use inew::New;

#[derive(New)]
struct MyAwesomeStruct {
    name: String,
    #[new(default)]
    entries: Vec<u32>,
    #[new(default)]
    some_values: std::collections::HashSet<u32>,
    #[new(default = custom_func())]
    custom_value: u32
}

fn custom_func() -> u32 {
    42u32
}

fn main() {
    MyAwesomeStruct::new("123".to_owned())
}

```

The #[new(default = ...)] attribute can take any valid Rust expression, such as 1 + 1 or vec![1], as its argument.

# Custom names and privacy

It is also possible to configure the privacy and rename the constructor using attributes.

# Privacy

```rust
#[derive(New)]
#[new(pub = false)]
struct MyStruct {
    x: u32,
}

fn main() {
    MyStruct::new(1) // now it's a private function
}
```

# Custom names

```rust
#[derive(New)]
#[new(rename = "create")]
struct MyStruct {
    x: u32,
}

fn main() {
    MyStruct::create(1)
}
```

# Generics and lifetimes

Generics and lifetimes are supported and work

## Generics

```rust
use inew::New;

#[derive(New)]
struct MyStruct<A, B> {
    x: u32,
    y: A,
    z: B,
}

fn main() {
    MyStruct::new(1u32, 2u64, 3u16)
}
```

## Lifetimes

```rust
use inew::New;

#[derive(New)]
struct MyStruct<'a> {
    x: u32,
    y: &'a u16,
}

fn main() {
    let y = 1u16;
    MyStruct::new(x, &y)
}
```

# Unnamed structures

Unnamed structures are fully supported as well

```rust
use inew::New;

#[derive(New)]
struct MyStruct(u32);

fn main() {
    MyStruct::new(1)
}
```

# Special thanks to

* Chat GPT-4, which helped me write all this documentation and correct a huge number of errors in the code
* Kristina, who was my inspiration
* Stable Diffusion, which helped me to create logo :-)

# Licensing

Licensed under either of Apache License, Version 2.0 or MIT license at your option.

# Contribution

Any contribution is welcome. Just write tests and submit merge requests

# Difference from derive-new

There is a very similar library with almost the same set of features and syntax. [derive-new](https://github.com/nrc/derive-new)
Below is a list of differences in the table.

| Feature                                 | INew | derive-new |
|-----------------------------------------|------|------------|
| Default values support                  | Yes  | Yes        |
| Generics and lifetimes support          | Yes  | Yes        |
| Enum support                            | No   | Yes        |
| Constructor privacy settings            | Yes  | No         |
| Constructor renaming                    | Yes  | No         |
| Unnamed structures support              | Yes  | Yes        |

# Related projects

## rust

[rust-derive-builder](https://github.com/colin-kiegel/rust-derive-builder)
[derive-new](https://github.com/nrc/derive-new)
[derive_more](https://github.com/JelteF/derive_more)

## java

[lombok](https://github.com/projectlombok/lombok)

## Non Library

Functionality is also built into the Scala, Kotlin, and Java languages for entities such
as  `case class`, `data class`, `record`




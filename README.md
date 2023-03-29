
# New -- library for generating constructors.
In Rust, writing constructors is common but can be repetitive and boring. This library simplifies the process, 
making it more enjoyable and freeing up time for more interesting tasks.

The purpose of this library is to cover the most basic and frequent case. If you want more complex generation, you
should probably take a look at  [rust-derive-builder](https://github.com/colin-kiegel/rust-derive-builder)

# How to add the library to your project?
Just add to Cargo.toml
```toml
new = "0.1.0"
```

# Мinimum supported Rust version
The minimum working version of Rust is `1.56.0`, which is also currently used for running tests in CI

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
But everything here is very obvious, all fields and types are known to compiler. Therefore, we can hand over constructor generation to a macro
```rust
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
initialization with `Default::default()` or `#[new(default = my_func_name)]` for initialization by calling
my_func_name().
Example of usage

```rust

#[derive(New)]
struct MyAwesomeStruct {
    name: String,
    #[new(default)]
    entries: Vec<u32>,
    #[new(default)]
    some_values: std::collections::HashSet<u32>,
    #[new(default = custom_func)]
    custom_value: u32
}

fn custom_func() -> u32 {
    42u32
}

fn main() {
    MyAwesomeStruct::new("123".to_owned())
}

```
Unfortunately, at the moment, functions with an explicit path are not supported, they need to be imported into the scope
explicitly. That's why path::to::custom_func will not work.



# Generics and lifetimes

Generics and lifetimes are supported and work

## Generics
```rust
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


# Special thanks to
* Chat GPT-4, which helped me write all this documentation and correct a huge number of errors in the code
* Kristina, who was my inspiration
* Stable Diffusion, which helped me to create logo :-)


# Related projects

## rust
[rust-derive-builder](https://github.com/colin-kiegel/rust-derive-builder)

## java
[lombok](https://github.com/projectlombok/lombok)

## Non Library
Functionality is also built into the Scala, Kotlin, and Java languages for entities such as  `case class`, `data class`, `record`




# INew – a library for generating constructors

In Rust, writing constructors is common but can be repetitive and boring. This library simplifies the process,
making it more enjoyable and freeing up time for more interesting tasks.

The purpose of this library is to cover the most basic and frequent case. If you want more complex generation, you
should probably take a look at  [rust-derive-builder](https://github.com/colin-kiegel/rust-derive-builder)

## How to add the library to your project?

Just add to Cargo.toml

```toml
[dependencies]
inew = "0.2.3"
```

## Мinimum supported Rust version

The library requires a minimum Rust version of `1.80.0`.

## Usage examples

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

### Default fields and custom functions for generating fields

If you don't want to pass all the fields, you can fill in some of the fields using annotations `#[new(default)]` for
initialization with `Default::default()` or `#[new(default = my_func_name())]` for initialization by calling
my_func_name().
Example of usage

```rust
use inew::New;

#[derive(New)]
struct MyStruct {
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
    let s = MyStruct::new("123".to_owned());
}
```

The #[new(default = ...)] attribute can take any valid Rust expression, such as 1 + 1 or vec![1], as its argument.

### Custom names and privacy

It is also possible to configure the privacy and rename the constructor using attributes.

#### Custom names

```rust
use inew::New;

#[derive(New)]
#[new(rename = "create")]
struct MyStruct {
    x: u32,
}

fn main() {
    let s = MyStruct::create(1);
}
```

#### Privacy

```rust
use inew::New;

#[derive(New)]
#[new(pub = false)]
struct MyStruct {
    x: u32,
}

fn main() {
    let s = MyStruct::new(1); // now it's a private function
}
```

### Generics and lifetimes

Generics and lifetimes are supported and work

#### Generics

```rust
use inew::New;

#[derive(New)]
struct MyStruct<Y, Z> {
    x: u32,
    y: Y,
    z: Z,
}

fn main() {
    let s = MyStruct::new(1u32, 2u64, 3u16);
}
```

#### Lifetimes

```rust
use inew::New;

#[derive(New)]
struct MyStruct<'a> {
    x: u32,
    y: &'a u16,
}

fn main() {
    let y = 1u16;
    let s = MyStruct::new(1, &y);
}
```

### Static lifetimes

```rust
use inew::New;

const NAME: &str = "John";

#[derive(New)]
struct MyStruct {
    name: &'static str,
}

fn main() {
    let s = MyStruct::new(NAME);
}
```

### Tuple structs

[Tuple structs](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#creating-different-types-with-tuple-structs) are fully supported as well

```rust
use inew::New;

#[derive(New)]
struct MyStruct(u32);

fn main() {
    let s = MyStruct::new(1);
}
```

### Unit-like structs

[Unit-like structs](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#defining-unit-like-structs) also work as expected

```rust
use inew::New;

#[derive(New)]
struct MyStruct;

fn main() {
    let s = MyStruct::new();
}
```

### Constant constructors

Derived constant constructors are also supported, but they come with some limitations, see below.

```rust
use inew::New;

#[derive(New)]
#[new(const = true)]
struct MyStruct {
    x: u32,
}

fn main() {
    const S: MyStruct = MyStruct::new(5);
}
```

Limitations for default values in constant constructors:

- Trait defaults like `#[new(default)]` attribute are not supported, since `Default` is not yet stable as a `const` trait.
- Macro defaults like `#[new(default = my_macro!())]` are supported as long as they expand to a constant expression, so any macro that does allocation is not supported.
- Function defaults like `#[new(default = my_function())]` are supported only if the function is `const`.
- Any struct with generics cannot have defaults of any kind.

### Unit and PhantomData

Fields with type `()` and `PhantomData` are always initialized with default values and skipped from the derived constructor, even for constant constructors.

```rust
use inew::New;
use std::marker::PhantomData;

#[derive(New)]
#[new(const = true)]
struct MyStruct<T> {
    x: (),
    y: PhantomData<T>,
}

fn main() {
    // Both cases below are valid
    let s: MyStruct<u32> = MyStruct::new();
    const S: MyStruct<u32> = MyStruct::new();
}
```

## Special thanks to

- Chat GPT-4, which helped me write all this documentation and correct a huge number of errors in the code
- Kristina, who was my inspiration
- Stable Diffusion, which helped me to create logo :-)

## Licensing

Licensed under either of Apache License, Version 2.0 or MIT license at your option.

## Contribution

Any contribution is welcome. Just write tests and submit merge requests

## Comparison with derive-new

There is a very similar library with almost the same set of features and syntax. [derive-new](https://github.com/nrc/derive-new)
Below is a list of differences in the table.

| Feature                                 | INew | derive-new |
|-----------------------------------------|------|------------|
| Default values support                  | Yes  | Yes        |
| Generics and lifetimes support          | Yes  | Yes        |
| Enum support                            | No   | Yes        |
| Constructor privacy settings            | Yes  | No         |
| Constructor renaming                    | Yes  | No         |
| Tuple structs support                   | Yes  | Yes        |
| Constant constructors support           | Yes  | No         |

## Related projects

### Rust libraries

[rust-derive-builder](https://github.com/colin-kiegel/rust-derive-builder)
[derive-new](https://github.com/nrc/derive-new)
[derive_more](https://github.com/JelteF/derive_more)

### Java libraries

[lombok](https://github.com/projectlombok/lombok)

### Non-library projects

Functionality is also built into the Scala, Kotlin, and Java languages for entities such
as  `case class`, `data class`, `record`

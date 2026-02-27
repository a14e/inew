# INew – a library for generating constructors

In Rust, writing constructors is common but can be repetitive and boring.
This library simplifies the process,
making the process more declarative and enjoyable, while freeing up time for more interesting tasks.

The purpose of this library is to cover the most basic and frequent use cases.
If you want more complex generation, you
should probably take a look at [rust-derive-builder](https://docs.rs/derive_builder/latest/derive_builder/)

## Table of contents

- [Adding the library to your project](#adding-the-library-to-your-project)
  - [Мinimum supported Rust version](#minimum-supported-rust-version)
  - [Breaking changes](#breaking-changes)
  - [Feature flags](#feature-flags)
    - [Standard library support](#standard-library-support)
    - [Default constructor visibility](#default-constructor-visibility)
- [Usage examples](#usage-examples)
  - [Structs](#structs)
    - [Tuple structs](#tuple-structs)
    - [Unit-like structs](#unit-like-structs)
  - [Enums](#enums)
  - [Use and type aliases](#use-and-type-aliases)
  - [Default fields and custom initializers](#default-fields-and-custom-initializers)
    - [Annotating default fields](#annotating-default-fields)
    - [Automatic default fields](#automatic-default-fields)
    - [Optional default values](#optional-default-values)
  - [Into trait helpers](#into-trait-helpers)
    - [Into paramaters](#into-paramaters)
    - [IntoIter parameters](#intoiter-parameters)
    - [Explicit IntoIter parameters](#explicit-intoiter-parameters)
  - [Custom constructor names](#custom-constructor-names)
    - [Custom names on structs](#custom-names-on-structs)
    - [Custom names on enums](#custom-names-on-enums)
    - [Enum constructors without prefixes](#enum-constructors-without-prefixes)
  - [Visibility of derived constructors](#visibility-of-derived-constructors)
    - [Default visibility](#default-visibility)
    - [Explicit visibility options](#explicit-visibility-options)
  - [Generics and lifetimes](#generics-and-lifetimes)
    - [Generics](#generics)
    - [Generic bounds](#generic-bounds)
    - [Lifetimes](#lifetimes)
    - [Static lifetimes](#static-lifetimes)
    - [Dynamic dispatch](#dynamic-dispatch)
  - [Constant constructors](#constant-constructors)
- [Special thanks to](#special-thanks-to)
- [License](#license)
- [Contributing](#contributing)
- [Comparison with derive-new](#comparison-with-derive-new)
- [Related projects](#related-projects)
  - [Rust libraries](#rust-libraries)
  - [Java libraries](#java-libraries)
  - [Programming languages](#programming-languages)

## Adding the library to your project

Add the dependency to your `Cargo.toml`.

```toml
[dependencies]
inew = "0.4.0"
```

### Minimum supported Rust version

The library requires a minimum Rust version of `1.80.0`.

### Breaking changes

The `v0.4.0` release has breaking changes which may affect older projects:

- Before `v0.4.0`, the default constructor visibility was `pub`.
This was changed to mimic default Rust visibility behavior.
The old behavior can be restored, see [Default constructor visibility](#default-constructor-visibility).

### Feature flags

### Standard library support

The `std` feature is enabled by default.
You can disable it by disabling all the default features, which adds support for `no_std`.

```toml
[dependencies]
inew = { version = "0.4.0", default-features = false }
```

### Default constructor visibility

The `public-default` feature is disabled by default, which makes all the derived constructors private (no preceding `pub` keyword).
Enabling it will change the default visibility of constructors to `pub`.

```toml
[dependencies]
inew = { version = "0.4.0", features = ["public-default"] }
```

## Usage examples

### Structs

Consider a struct with several different fields, along with a conventional constructor implementation, which may look like this:

```rust
struct MyStruct {
    x: u32,
    y: u16,
    z: String,
    field: bool,
    another_field: f32
}

impl MyStruct {
    pub fn new(x: u32, y: u16, z: String, field: bool, another_field: f32) -> Self {
        Self {
            x,
            y,
            z,
            field,
            another_field,
        }
    }
}

fn main() {
    let s = MyStruct::new(1, 2, "Z".to_string(), true, 3.14);
}
```

Having to create such a big `impl` block for a common and predictable pattern is repetitive and often a waste of time.
Instead, the `New` macro can be used to generate the constructor implementation and avoid having to write so much boilerplate:

```rust
use inew::New;

#[derive(New)]
struct MyStruct {
    x: u32,
    y: u16,
    z: String,
    field: bool,
    another_field: f32
}

fn main() {
    let s = MyStruct::new(1, 2, "Z".to_string(), true, 3.14);
}
```

By default:

- The parameter list matches the struct fields in declaration order.
- All the derived struct constructors will have the `new` name.
See [Custom constructor names](#custom-constructor-names) for renaming options.

#### Tuple structs

Tuple structs are fully supported as well, and they work the same as normal structs.

```rust
use inew::New;

#[derive(New)]
struct MyStruct(u32);

fn main() {
    let s = MyStruct::new(1);
}
```

Parameter names are numbers prefixed by and underscore, like `_0`, `_1`, `_2`, and so on.

#### Unit-like structs

Unit-like structs also work as expected, and they generate a parameterless constructor:

```rust
use inew::New;

#[derive(New)]
struct MyStruct;

fn main() {
    let s = MyStruct::new();
}
```

### Enums

The macro can also derive constructors for enums.
A constructor will be generated for each variant:

```rust
use inew::New;

#[derive(New)]
enum MyEnum {
    None,
    Point { x: u32, y: u32 },
    Color(u8, u8, u8),
}

fn main() {
    let n = MyEnum::new_none();
    let p = MyEnum::new_point(1, 2);
    let c = MyEnum::new_color(1, 2, 3);
}
```

They work similarly to struct constructors, but enum constructor names have a few differences:

- Each constructor is prefixed with `new_`.
See [Custom constructor names](#custom-constructor-names) for renaming options.
- The variant name is added after the prefix, but converted to `snake_case`.

Structs and enums have feature parity, so most of the examples below will use structs to keep them short.

### Propagation of lint attributes

The macro propagates lint attributes from the struct definition to the generated constructors.
For example, the following code will NOT generate a warning for Clippy's `too_many_arguments` lint:

```rust
#![warn(clippy::too_many_arguments)]

use inew::New;

#[allow(clippy::too_many_arguments)]
#[derive(New)]
struct MyStruct {
    a: i32,
    b: i32,
    c: i32,
    d: i32,
    e: i32,
    f: i32,
    g: i32,
    h: i32,
}
```

### Use and type aliases

The macro works fine most of the time when used with `use` and `type` aliases:

```rust
use inew::New;
use std::string::String as S;

type X = u32;

#[derive(New)]
struct MyStruct {
    x: X,
    name: S,
}

fn main() {
    let s = MyStruct::new(1, "John".to_string());
}
```

But there are a few edge cases where the macro will not work as expected with them, which will be explained in more detail when it becomes relevant.

### Default fields and custom initializers

#### Annotating default fields

Fields can be omitted from the derived constructor by annotating them with the following:

- `#[new(default)]` initializes the field using `Default::default()`.
- `#[new(default = <expression>)]` initializes the field using the provided expression.
It can take any valid Rust expression as its argument, such as `1 + 1` or `vec![1]`.

```rust
use inew::New;
use std::collections::HashSet;

macro_rules! custom_macro {
    () => {
        true
    };
}

#[derive(New)]
struct MyStruct {
    name: String,
    #[new(default)]
    some_values: HashSet<u32>,
    #[new(default = 1 + 2)]
    entry_count: usize,
    #[new(default = custom_func())]
    custom_value: u32,
    #[new(default = custom_macro!())]
    is_enabled: bool,
}

fn custom_func() -> u32 {
    42u32
}

fn main() {
    let s = MyStruct::new("123".to_owned());
}
```

#### Automatic default fields

There are two special cases of fields that are automatically skipped from constructors and initialized with their only possible value:

- Unit fields, which are initialized as `()`.
- `PhantomData<T>` fields, which are initialized as `PhantomData`.

```rust
use inew::New;
use std::marker::PhantomData;

#[derive(New)]
struct MyStruct<T> {
    unit: (),
    phantom: PhantomData<T>,
}

fn main() {
    let s = MyStruct::<u32>::new();
}
```

Due to procedural macro limitations, `use` aliases such as `use std::marker::PhantomData as PD;` cannot be automatically detected.
For these cases, you'll have to be explicit and use the `#[new(default)]` attribute instead.

```rust
use inew::New;
use std::marker::PhantomData as PD;

#[derive(New)]
struct MyStruct<T> {
    #[new(default)]
    phantom: PD<T>,
}

fn main() {
    let s = MyStruct::<u32>::new();
}
```

#### Optional default values

Default fields can be annotated with `#[new(optional)]` to make them optional.
This turns the parameter into `Option<T>` instead of `T`.

```rust
use inew::New;
use std::collections::HashSet;

macro_rules! custom_macro {
    () => {
        true
    };
}

#[derive(New)]
struct MyStruct {
    name: String,
    #[new(optional)]
    some_values: HashSet<u32>,
    #[new(default = 1 + 2, optional)]
    entry_count: usize,
    #[new(default = custom_func(), optional)]
    custom_value: u32,
    #[new(default = custom_macro!(), optional)]
    is_enabled: bool,
}

fn custom_func() -> u32 {
    42u32
}

fn main() {
    let s = MyStruct::new("123".to_owned(), None, None, None, None);
    let s2 = MyStruct::new("abc".to_owned(), Some(HashSet::new()), Some(10), Some(100), Some(false));
}
```

Note that `optional` acts as a modifier of the `default` attribute, meaning that it can't be used alone for most cases.
The only exception is `#[new(optional)]`, which implies and doesn't require adding `default`.

### Into trait helpers

Constructor parameters can be made more flexible by automatically converting arguments using the `Into` or `IntoIterator` traits.

Only fields without default values can participate in these conversions.
In other words, a field's `#[new(...)]` may use at most one of:

- `default`
- `into`
- `into_iter`

#### Into paramaters

Annotating a field with `#[new(into)]` changes the generated parameter type from `T` to `impl Into<T>`, and calls `.into()` internally.

```rust
use inew::New;

#[derive(New)]
struct MyStruct {
    #[new(into)]
    name: String,
}

fn main() {
    let s = MyStruct::new("John");
}
```

The example above allows passing `&str`, `String`, or any other type that implements `Into<String>`.

#### IntoIter parameters

For collection fields, `#[new(into_iter)]` allows accepting any `IntoIterator<Item = T>` and collecting into the target collection type by calling `into_iter().collect()` internally.

```rust
use inew::New;

#[derive(New)]
struct MyStruct {
    #[new(into_iter)]
    x: Vec<u32>,
}

fn main() {
    let s = MyStruct::new(Some(5));
    let t = MyStruct::new(None);
    let u = MyStruct::new([1, 2, 3]);
}
```

The example above works because:

- `<u32>` is part of the `x` field's type, so it could be inferred automatically.
- `Option<T>` implements `IntoIterator<Item = T>`.
- Arrays of type `[T; N]` implement `IntoIterator<Item = T>`.
- `Vec<T>` implements `FromIterator<T>`.

#### Explicit IntoIter parameters

In more complex cases (e.g., type aliases), item type inference may fail.
For such cases, you can specify the iterator item type explicitly using `#[new(into_iter = <item_type>)]`:

```rust
use inew::New;

type MyVector = Vec<u32>;

#[derive(New)]
struct MyStruct {
    #[new(into_iter = u32)]
    x: MyVector,
}
```

### Custom constructor names

The derived constructor's name can be customized using the `#[new(rename = <custom_name>)]` option.

#### Custom names on structs

For structs, `#[new(rename = <custom_name>)]` replaces the default `new` function name with `<custom_name>`.

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

#### Custom names on enums

For enums, using `#[new(rename = <custom_prefix>)]` replaces only the `new` prefix, not the full function name.

```rust
use inew::New;

#[derive(New)]
#[new(rename = "create")]
enum MyEnum {
    First,
    Second { x: u32 },
    Third (u32),
}

fn main() {
    let f = MyEnum::create_first();
    let s = MyEnum::create_second(2);
    let t = MyEnum::create_third(3);
}
```

#### Enum constructors without prefixes

If desired, the `no_prefix` option can be used to remove the prefix from generated `enum` constructors.
Note that passing `#[new(rename = "")]` instead is not allowed.

```rust
use inew::New;

#[derive(New)]
#[new(no_prefix)]
enum MyEnum {
    First,
    Second { x: u32 },
    Third (u32),
}

fn main() {
    let f = MyEnum::first();
    let s = MyEnum::second(2);
    let t = MyEnum::third(3);
}
```

`no_prefix` cannot be used on structs, or together with `rename = ...`

### Visibility of derived constructors

#### Default visibility

Constructor visibility can be controlled at the type level using the `#[new(...)]` attribute.

If no visibility option is specified, the constructor visibility depends on the `public-default` feature:

- Without `public-default`: constructors are private (no `pub` keyword).
- With `public-default`: constructors are `pub`.

See [Default constructor visibility](#default-constructor-visibility) for information on how to enable this feature.

#### Explicit visibility options

Visibility options allow specifying the constructor visibility with syntax similar to Rust's `pub` visibility keywords.

The following forms are supported:

| Attribute                    | Derived visibility                             |
|------------------------------|------------------------------------------------|
| *(nothing)*                  | Private or `pub` (depends on `public-default`) |
| `#[new(pub)]`                | `pub`                                          |
| `#[new(pub = true)]`         | `pub`                                          |
| `#[new(pub = false)]`        | Private                                        |
| `#[new(pub(crate))]`         | `pub(crate)`                                   |
| `#[new(pub(super))]`         | `pub(super)`                                   |
| `#[new(pub(self))]`          | `pub(self)`                                    |
| `#[new(pub(in <ancestor>))]` | `pub(in <ancestor>)`                           |

The following example showcases all the supported visibility options:

```rust
use inew::New;

// Depends on the `public-default` feature
#[derive(New)]
struct DefaultVisibility {
    x: u32,
}

// pub
#[derive(New)]
#[new(pub)]
struct ExplicitPub {
    x: u32,
}

// pub (explicit)
#[derive(New)]
#[new(pub = true)]
struct ExplicitPubTrue {
    x: u32,
}

// private
#[derive(New)]
#[new(pub = false)]
struct ExplicitPrivate {
    x: u32,
}

// crate
#[derive(New)]
#[new(pub(crate))]
struct CrateVisible {
    x: u32,
}

mod outer {
    use inew::New;

    // super
    #[derive(New)]
    #[new(pub(super))]
    pub struct SuperVisible {
        x: u32,
    }

    // self
    #[derive(New)]
    #[new(pub(self))]
    pub struct SelfVisible {
        x: u32,
    }

    pub mod inner {
        use inew::New;

        // in <ancestor> (crate is always a valid example)
        #[derive(New)]
        #[new(pub(in crate))]
        pub struct RestrictedToOuter {
            x: u32,
        }
    }
}
```

### Generics and lifetimes

The `New` derive macro fully preserves information about generics and lifetimes.
All type parameters, lifetime parameters, and bounds are copied to the derived `impl` block.

#### Generics

Generic type parameters are supported out of the box.

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

More complex cases with nested generics are also supported:

```rust
use inew::New;

#[derive(New)]
struct NestedStruct<Y, Z> {
    y: Y,
    z: Z,
}

#[derive(New)]
struct MyStruct<Y, Z> {
    x: NestedStruct<Y, Z>,
}

fn main() {
    let res = MyStruct::new(NestedStruct::new(1, "z"));
}
```

#### Generic bounds

Generic bounds are also copied to the derived `impl` block.

```rust
use inew::New;

#[derive(New)]
struct MyStruct<T: Clone> {
    x: u32,
    y: T,
}

#[derive(New)]
struct AnotherStruct<T>
where
    T: Clone,
{
    x: u32,
    y: T,
}

fn main() {
    let s = MyStruct::new(1u32, 2u64);
    let s2 = AnotherStruct::new(1u32, 2u64);
}
```

#### Lifetimes

Lifetime parameters are preserved and propagated to the constructor.

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

#### Static lifetimes

Static references (`'static T`) require no special handling.

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

#### Dynamic dispatch

Fields using trait objects (dynamic dispatch) are fully supported, including borrowed `dyn` traits with lifetimes.

```rust
use inew::New;

#[derive(New)]
struct MyStruct<'a> {
    function: &'a dyn Fn(f32, f32) -> f32,
}

fn main() {
    let add = |x: f32, y: f32| x + y;
    let subtract = |x: f32, y: f32| x - y;

    let mut s = MyStruct::new(&add);
    (s.function)(1.0, 2.0);

    s.function = &subtract;
    (s.function)(1.0, 2.0);
}
```

### Constant constructors

Constructors can be generated as `const` functions using `#[new(const)]`, which allows usage in them in constant contexts.

```rust
use inew::New;

#[derive(New)]
#[new(const)]
struct MyStruct {
    x: u32,
}

fn main() {
    const S: MyStruct = MyStruct::new(5);
}
```

Constant constructors share the same limitiations as Rust's `const fn`.
Important limitations are:

- `Default` is not yet stable as a `const` trait, so `#[new(default)]` is not supported.
- Defaults of the form `#[new(default = expression)]` are supported as long as the expression is valid in a constant context.
This is true for most macros without allocation and `const` functions.
- If the struct or enum has generics, default values are not supported for any case.
- Since the `Into` and `ÌntoIter` traits are not `const` traits, the `#[new(into)]` and `#[new(into_iter)]` attributes are not supported.

## Special thanks to

- ChatGPT-4, which helped me write all the documentation for the first version and correct a huge number of errors in the code, during the early phases
- Anna, who was my inspiration
- Stable Diffusion, which helped me to create logo :-)

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.
A copy of the licenses is available in the [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) files.

## Contributing

Any contribution is welcome.
Just write tests and submit merge requests.

## Comparison with derive-new

There is a very similar library with similar features and sytax, [derive-new](https://docs.rs/derive_new/latest/derive_new/).
Below is a list of differences in the table:

| Feature                         | INew | derive-new |
|---------------------------------|------|------------|
| Tuple structs support           | Yes  | Yes        |
| Enum support                    | Yes  | Yes        |
| Propagation of lint attributes  | Yes  | Yes        |
| Default values for fields       | Yes  | Yes        |
| Optional default values         | Yes  | No         |
| Into parameters support         | Yes  | Yes        |
| IntoIter parameters support     | Yes  | Yes        |
| Constructor visibility settings | Yes  | Yes        |
| Custom constructor names        | Yes  | No         |
| Generics and lifetimes support  | Yes  | Yes        |
| Constant constructor generation | Yes  | No         |

## Related projects

### Rust libraries

- [rust-derive-builder](https://docs.rs/derive_builder/latest/derive_builder/)
- [derive-new](https://docs.rs/derive_new/latest/derive_new/)
- [derive_more](https://docs.rs/derive_more/latest/derive_more/)

### Java libraries

- [lombok](https://projectlombok.org/)

### Programming languages

Similar functionality is also built into other programming languages, such as:

- Java's [`record`](https://docs.oracle.com/en/java/javase/25/language/records.html)
- Kotlin's [`data class`](https://kotlinlang.org/docs/data-classes.html)
- Python's [`@dataclass`](https://docs.python.org/3/library/dataclasses.html)
- Scala's [`case class`](https://docs.scala-lang.org/tour/case-classes.html)

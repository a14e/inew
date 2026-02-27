use inew::New;

#[derive(New)]
struct A {
    #[new(default, default)]
    x: u32,
}

fn main() {}

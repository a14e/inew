use inew::New;

#[derive(New)]
struct A {
    #[new(default, default = 5)]
    x: u32,
}

fn main() {}

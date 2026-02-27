use inew::New;

#[derive(New)]
struct A {
    #[new(optional, optional = 5)]
    x: u32,
}

fn main() {}

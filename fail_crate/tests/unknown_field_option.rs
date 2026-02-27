use inew::New;

#[derive(New)]
struct A {
    #[new(what)]
    x: u32,
}

fn main() {}

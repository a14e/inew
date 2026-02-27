use inew::New;

#[derive(New)]
struct A {
    #[new(optional, into)]
    x: u32
}

fn main() {}

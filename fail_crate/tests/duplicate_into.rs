use inew::New;

#[derive(New)]
struct A {
    #[new(into, into)]
    x: u32
}

fn main() {}

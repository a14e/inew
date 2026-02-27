use inew::New;

#[derive(New)]
struct A {
    #[new(option)]
    x: u32
}

fn main() {}

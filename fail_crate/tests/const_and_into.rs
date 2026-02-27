use inew::New;

#[derive(New)]
#[new(const)]
struct A {
    #[new(into)]
    x: u32
}

fn main() {}

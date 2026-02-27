use inew::New;

#[derive(New)]
#[new(const)]
struct A {
    #[new(optional)]
    x: u32
}

fn main() {}

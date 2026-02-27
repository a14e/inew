use inew::New;

#[derive(New)]
#[new(const)]
struct A {
    #[new(default)]
    x: u32
}

fn main() {}

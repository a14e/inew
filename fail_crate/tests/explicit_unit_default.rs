use inew::New;

#[derive(New)]
struct A {
    #[new(default)]
    x: (),
}

fn main() {}

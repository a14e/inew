use inew::New;

#[derive(New)]
struct A {
    #[new(into_iter)]
    x: u32
}

fn main() {}

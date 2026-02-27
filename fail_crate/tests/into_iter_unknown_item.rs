use inew::New;

#[derive(New)]
#[new(const)]
struct A {
    #[new(into_iter)]
    x: u32
}

fn main() {}

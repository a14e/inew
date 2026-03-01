use inew::New;

#[derive(New)]
struct A {
    #[new(into_iter, into_iter)]
    x: Vec<u32>,
}

fn main() {}

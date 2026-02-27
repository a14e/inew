use inew::New;

#[derive(New)]
struct A {
    #[new(into, into_iter)]
    x: Vec<u32>
}

fn main() {}

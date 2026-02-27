use inew::New;

#[derive(New)]
struct A {
    #[new(optional, into_iter)]
    x: Vec<u32>
}

fn main() {}

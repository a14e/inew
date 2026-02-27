use inew::New;

#[derive(New)]
struct A {
    #[new(default, into_iter)]
    x: Vec<u32>
}

fn main() {}

use inew::New;

#[derive(New)]
#[new(const)]
struct A {
    #[new(into_iter)]
    x: Vec<u32>
}

fn main() {}

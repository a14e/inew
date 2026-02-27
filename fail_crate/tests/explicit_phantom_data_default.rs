use inew::New;
use std::marker::PhantomData;

#[derive(New)]
struct A<T> {
    #[new(default)]
    x: PhantomData<T>,
}

fn main() {}

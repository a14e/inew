use inew::New;
use std::marker::PhantomData;

#[derive(New)]
struct A<T> {
    #[new(optional)]
    x: PhantomData<T>,
}

fn main() {}

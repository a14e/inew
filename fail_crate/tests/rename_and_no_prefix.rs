use inew::New;

#[derive(New)]
#[new(rename = "create", no_prefix)]
enum A {}

fn main() {}

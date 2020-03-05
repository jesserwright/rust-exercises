// #[derive(Copy, Clone)]
struct C;

struct B<'b> {
    c: &'b C,
}

struct A<'a> {
    b: B<'a>, // refrence to same
    c: &'a C,
}

// all refer to the same lifetime

fn main() {
    let c1 = C;
    let _ = A::new(&c1);
}

impl<'a> A<'a> {
    fn new(c: &'a C) -> A<'a> {
        A { c, b: B { c } }
    }
}

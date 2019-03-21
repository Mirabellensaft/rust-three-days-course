// Where syntax(preferred)
fn generic_where<Stringish>(thing: Stringish) -> Stringish
where Stringish: AsRef<str> {
    thing
}

// Inline syntax
fn generic_inline<S: AsRef<str>>(thing: S) -> S {
    thing
}

// Enums too!
struct GenericStruct<A> {
    value: A,
}

fn main() {
    let foo = "foo";
    generic_inline(foo);
    generic_where(foo);
}

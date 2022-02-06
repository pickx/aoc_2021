trait Opt<U>: Into<Option<U>> {}
impl<T: Into<Option<U>>, U> Opt<U> for T {}

fn optional<T: Opt<usize>>(arg: T) {
    println!("{:?}", arg.into())
}

fn main() {
    optional(5)
}

fn main() {
    println!("Hello, world!");
    static_str();
}

fn static_str() {
    let s = "this is string reference"; // type &static str (slice type)
    println!("{}", s);
    let s = "foo
    bar"; // this will add some editor space
    assert_eq!("foo\n    bar", s);
}

fn to_string() {
    let mut s = "this is String type".to_string(); // use to_string to convert to String type
    s.push_str(". String size is not fixed");
    println!("{}", s);
}

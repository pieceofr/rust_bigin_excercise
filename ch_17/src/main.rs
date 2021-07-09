use std::net::TcpStream;

fn main() {
    println!("Hello, world!");
    static_str();
    to_string();
    string_to_str();
    indexing();
    slice();
    concatenate();
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

fn take_str(words: &str) {
    println!("{}", words);
}
fn string_to_str() {
    let s = "hello".to_string();
    take_str(&s); // String 使用 & 變成 &string

    // 例外: connect 的參數 &string 是作為 trait , String 要用 &*String 來轉化為&str
    let _ = TcpStream::connect("127.0.0.1:3000");
    let address_string = "127.0.0.1:300".to_string();
    let _ = TcpStream::connect(&*address_string);
}

fn indexing() {
    let hachiko = "忠犬ハチ公";
    for (i, b) in hachiko.as_bytes().iter().enumerate() {
        println!("{} byte = {}", i, b); // print as bytes and it has 14 bytes in 5 char
    }
    println!("");
    let mut ic = 0;
    for c in hachiko.chars() {
        // it seems str chars method does not support iter ... (does't it?)
        println!("{} char = {}", ic, c); // print 5 chars
        ic = ic + 1;
    }
    println!("")
}

fn slice() {
    let hachiko = "hachiko";
    let hachi = &hachiko[0..5];
    println!("hachi={}", hachi);

    let jphachiko = "忠犬ハチ公";
    // thread 'main' panicked at 'byte index 2 is not a char boundary; it is inside '忠' (bytes 0..3) of `忠犬ハチ公`',
    //let jphachi = &jphachiko[0..2];
    let _jphachi = &jphachiko[0..3]; // 忠 is 0..3
}

fn concatenate() {
    // &str + &str -> can not at the moment, may find the solution later
    // let hello = "hello";
    // let world = "world";
    // let hello_world = hello + world;
    // Error!!! cannot be used to concatenate two `&str` strings

    // String + &str -> String
    let hello1 = "hello".to_string();
    let world1 = "world";
    let hello_world1 = hello1 + " " + world1;
    println!("{}", hello_world1);

    // String + String -> String must force convert 2nd String to a &str
    let hello2 = "hello".to_string();
    let world2 = "world".to_string();
    let hello_world2 = hello2 + " " + &world2; // force world to be a &str
    println!("{}", hello_world2);

    //  &str + String -> String
    // let hello3 = "hello";
    // let world3 = "world".to_string();
    // let hello_world3 = hello3 + world3;
    //  `+` cannot be used to concatenate a `&str` with a `String`
}

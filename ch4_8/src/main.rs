fn main() {
    println!("Hello, world!");
    //borrow01();
    //borrow02();
    //borrow03();
    //borrow04();
    let reference_to_nothing = dangle();
}
/*
fn borrow01() {
    let mut v = vec![1, 2, 3];
    for i in &v {
        println!("{}", i);
        v.push(34); // remove comment to see compile error
    }
}
*/
fn borrow02() {
    let y: &i32;
    let x = 5;
    y = &x;
    println!("{}", y);
}

fn borrow03() {
    let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}

fn borrow04() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM
    //println!("r1={}, r2={}, r3={}",r1, r2,r3); // mutable, immutable at the same time
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
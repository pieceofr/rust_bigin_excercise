fn main() {
    println!("Hello, world!");
    ignore();
    at_bind();
}

fn ignore() {
    let x = String::from("  hello  ");
    //let x = String::from("  hello  ").trim(); //Error  let _ = String::from("  hello  ").trim();
    let y = x.trim();
    println!("x={}, trim = {}", x, y);
    let _ = String::from("  hello  ").trim();
}

fn foo() -> i32 {
    22
}
fn bar(x: &i32) -> &i32 {
    x
}
/// Error E0716:A temporary value is being dropped while a borrow is still in active use.

fn e0716_error() {
    let p = bar(&foo()); //
                         // 執行 foo()  時會用一個 temporary 把 return value 存起來
                         // bar 是借用這個 temporary. 所以沒問題. 但是, temporay value 會在 staement 結束後被釋放
                         // 這個 statement 結束就是 let p 的結束. 這個 p 也是借用 所以當 temporay 被釋放後 就無所指了.
                         // 因此在 let p 之後使用 p 就出有問題了. 所以如果不使用到 p 就可以compile 過
                         //let _q = *p;
                         //println!("q={}", q);
}

fn e0716_no_error() {
    let value = foo(); // 把temporay 存起來在 value
    let p = bar(&value);
    let q = *p;
}

fn e0716_no_error2() {
    let value = &foo(); // 假如 temporay 立刻被借用並且被assign 一個變數. 那個tempory 在 statement 結束後不會消失
                        // 是直到 block 結束才消失
    let p = bar(value);
    let _q = *p;
    let _value2 = (&foo(), &foo()); //此規則也可用在 aggregate structure 上 . ie tuple or struct
}
enum OptionalTuple {
    Value(i32, i32, i32),
    Missing,
}
fn multiple_neglect() {
    let x = OptionalTuple::Value(5, -2, 3);
    match x {
        OptionalTuple::Value(..) => println!("Got a tuple!"), // use .. to neglect value
        OptionalTuple::Missing => println!("No such luck."),
    }
}

fn at_bind() {
    let x = 5;
    match x {
        e @ 1...5 => println!("e={}", e),
        _ => println!("anything"),
    }
}

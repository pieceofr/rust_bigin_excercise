fn main() {
    println!("Hello, Generic Type!");
    // std 的 Option 就是一個 泛型
    let x: Option<i32> = Some(5);
    // let x: Option<f64> = Some(5); Fail 因為 雙邊不同型別
}
// Generic Function , 對 T 的型別泛型 x與y 都是 T 型
fn take_anything<T>(x: T, y: T) {}

// 接受兩個泛型 T & U, x 是 T , y 是 U 型
fn take_two_thing<T, U>(x: T, y: U) {}

// Generic Struct
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn swap(&mut self) {
        std::mem::swap(&mut self.x, &mut self.y);
    }
}

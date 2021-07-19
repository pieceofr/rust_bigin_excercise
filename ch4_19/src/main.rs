use std::fmt::Debug;

fn main() {
    println!("Trait Example");
    let c = Circle {
        x: 0.0f64,
        y: 0.0f64,
        radius: 1.0f64,
    };

    let s = Square {
        x: 0.0f64,
        y: 0.0f64,
        side: 1.1f64,
    };

    print_area(c);
    print_area(s);
    // print_area(6.4);  Error 因為print_area 只可以用在HasArea 這個泛型
    // 這樣就限定了輸入的種類(此案就限制了shape)

    print_area(5); // 我們對 i32 做了 hasArea 泛型設計. 但這並非好設計

    println!("{:?}", DeriveStruct)
}

fn print_area<T: HasArea>(shape: T) {
    println!("This shapre area is {}", shape.area());
}

struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

struct Square {
    x: f64,
    y: f64,
    side: f64,
}

// 如果想要有 trait function 就要 implement trait 下的 functions
// Trait 可被用在范型
trait HasArea {
    // Function 特徵 HasArea
    fn area(&self) -> f64;
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

impl HasArea for Square {
    // 為square 實做特徵 HasArea
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

// 我們其實也可以對primitive 做泛型 fn

impl HasArea for i32 {
    fn area(&self) -> f64 {
        *self as f64
    }
}

//使用兩個Trait 限制的 function
fn foo<T: Clone + Debug>(x: T) {
    x.clone();
    println!("{:?}", x);
}

// 多個 trait 和泛型 params
fn foo2<T: Clone, K: Clone + Debug>(x: T, y: K) {
    x.clone();
    y.clone();
    println!("{:?}", y);
}

// 上面也可以用 where 來組句
fn bar<T, K>(x: T, y: K)
where
    T: Clone,
    K: Clone + Debug,
{
    x.clone();
    y.clone();
    println!("{:?}", y);
}

// 接下來示範複雜的 where 方式
trait ConvertTo<Output> {
    fn convert(&self) -> Output;
}

// 為i32 implement 了 ConverTo<i64> 這個 trait
impl ConvertTo<i64> for i32 {
    fn convert(&self) -> i64 {
        *self as i64
    }
}
/// --- > 以下不明其意

// normal 是一個接受泛型 ConvertTo<i64>的 function, 他接受擁有 ConvertTo<i64> 的 reference 最後輸出 i64
fn normal<T: ConvertTo<i64>>(x: &T) -> i64 {
    x.convert()
}
// where 句子不一定要 T 也可以限定為 i32

fn inverse<T>() -> T
where
    i32: ConvertTo<T>,
{
    42.convert()
}
// 如果有 Default, 則可以使用 Default
struct UseDefault;

trait Foo {
    fn is_valid(&self) -> bool;
    fn is_invalid(&self) -> bool {
        // 已經 implement 好
        !self.is_valid()
    }
}

impl Foo for UseDefault {
    fn is_valid(&self) -> bool {
        println!("Only implement unimplemented is_invalid");
        true
    }
}

// 就算已經有 Default implement , 也可以 Override
struct OverrideDefault;

impl Foo for OverrideDefault {
    fn is_valid(&self) -> bool {
        println!("implemt is_valid for OverrideDefault");
        true
    }
    fn is_invalid(&self) -> bool {
        println!("Override default implementation for Override Default");
        true
    }
}

// Trait 也可以繼承 ie 這邊 foobar 繼承 foo
struct Baz;

trait FooBar: Foo {
    fn foobar(&self);
}

// 假如忘記 implement Fool , 會有 Error:  the trait `Foo` is not implemented for `Baz`
impl Foo for Baz {
    fn is_valid(&self) -> bool {
        println!("Foo for is_valid");
        true
    }
}

impl FooBar for Baz {
    fn foobar(&self) {
        println!("foobar for FooBar");
    }
}

//Rust 自動幫忙實做常用 trait
#[derive(Debug)]
struct DeriveStruct;

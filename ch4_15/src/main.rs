/// Method Syntax
///
fn main() {
    println!("Hello, world!");
    let c = Circle {
        x: 0.0,
        y: 0.0,
        radius: 2.0,
    };
    let mut c1 = Circle {
        x: 0.0,
        y: 0.0,
        radius: 2.0,
    };
    println!("{}", c.area());
    println!("{}", c1.area_mut());
    println!("{}", c.area_self());

    // Builder Example
    let c = CircleBuilder::new().x(1.0).y(2.0).radius(2.0).finalize();

    println!("area: {}", c.area());
    println!("x: {}", c.x);
    println!("y: {}", c.y);
}

struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    // parameter can be self (if array), &self (reference) or &mut selt (mutable ref)
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
    fn area_mut(&mut self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }

    fn area_self(self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
    // Chaining method call
    // 因為可以達成 foo.bar().baz() 這樣的呼叫法
    fn grow(&self, increment: f64) -> Circle {
        Circle {
            x: self.x,
            y: self.y,
            radius: self.radius + increment,
        }
    }
}
// 可以分開或是像上面一樣合併 implement 不同 method
impl Circle {
    fn reference(&self) {
        println!("taking self by reference!");
    }
}

// Builder 的作法

struct CircleBuilder {
    x: f64,
    y: f64,
    radius: f64,
}

impl CircleBuilder {
    fn new() -> CircleBuilder {
        CircleBuilder {
            x: 0.0,
            y: 0.0,
            radius: 1.0,
        }
    }
    // mut self 宣告了要把 circleBuilder 當作 &mut CircleBuilder
    fn x(&mut self, coordinate: f64) -> &mut CircleBuilder {
        self.x = coordinate;
        self
    }
    fn y(&mut self, coordinate: f64) -> &mut CircleBuilder {
        self.y = coordinate;
        self
    }
    fn radius(&mut self, coordinate: f64) -> &mut CircleBuilder {
        self.radius = coordinate;
        self
    }
    // 最後 reassign CircleBuild 為 &self
    fn finalize(&self) -> Circle {
        Circle {
            x: self.x,
            y: self.y,
            radius: self.radius,
        }
    }
}

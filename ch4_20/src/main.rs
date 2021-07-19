fn main() {
    println!("Hello, Drop!");
    let firecracker = Firework { strength: 1 }; // Drop in reverse order, first in, last out
    let tnt = Firework { strength: 100 };
}

struct Firework {
    strength: i32,
}

impl Drop for Firework {
    fn drop(&mut self) {
        println!("BOOM times {} !!!", self.strength);
    }
}

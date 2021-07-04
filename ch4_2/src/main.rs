fn main() {
    println!("Hello, world!");
    //let x:i32=divergence();
    //let y:String=divergence();
    let a = true;
    println!("copyA:{},a={}",copytrait(a), a);
}
// 發散函式
fn divergence()->! {
   panic!("This function never return");
}

fn copytrait(x:bool)->bool{
    !x
}


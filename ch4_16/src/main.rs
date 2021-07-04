fn main() {
    println!("Hello, world!");
    let v = vec![1, 2, 3, 4, 5];

    let idx: usize = 2;
    println!("idx must be usize. idex {} is {}", idx, v[idx]);
    let v_init = vec![1; 5];
    // iterating : for loop 1
    // no matter vec is mut or imut, we can use &v_init
    for (i, e) in (&v_init).iter().enumerate() {
        println!("loop1 elem {} value is {}", i, e);
    }
    // vec must be mut so that we can use &mut
    let mut v2 = vec![5, 6, 7, 8];
    for (i, e) in (&mut v2).iter().enumerate() {
        println!("loop2 elem {} value is {}", i, e);
    }
    for (i, e) in v.iter().enumerate() {
        println!("loop3 elem {} value is {}", i, e);
    }
    println!("{}", v[0]); // Still keep ownership , i think ownership does not move to new binding
}

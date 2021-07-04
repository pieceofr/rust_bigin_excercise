struct Foo<'a> {
    x: &'a i32,
}

fn main(){
    //lifetime_scope();
}

fn lifetime_scope() {
    let x;                    // -+ x goes into scope
                              //  |
    {                         //  |
        let y = &5;           // ---+ y goes into scope
        let f = Foo { x: y }; // ---+ f goes into scope
        x = &f.x;             //  | | error here
    }                         // ---+ f and y go out of scope
                              //  |
    println!("{}", x);        //  |
}                             // -+ x goes out of scope

fn lifetime_auto(){

}
fn get_str()->&str{ // return borrow value but no input borrow value
    "aaa"
}
fn frob(s:&str , t:&str)->&str{ // does not specify it borrow from s or t
    "aaa"
}
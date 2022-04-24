// https://doc.rust-lang.org/rust-by-example/fn/closures.html 

fn main(){
    let closure_annotated = |i: i32| -> i32{i+1};
    let closure_inferred  = |i| {i+1};

    let i=1i32;

    println!("closure_annotated : {}", closure_annotated(i));
    println!("closure_inferred : {}", closure_inferred(i));
    let one = || 1i32;
    println!("one closure : {}", one());

    //////////////////////////////////////////////////////////
    
    use std::mem;

    let color = String::from("green");

    let print = || println!("color : {}", color);
    print();

    let _reborrow = &color;
    print();

    /////////////////////////////////////////////////////////

    /*
        Fn: the closure uses the captured value by reference (&T)
        FnMut: the closure uses the captured value by mutable reference (&mut T)
        FnOnce: the closure uses the captured value by value (T)
    */
    
    fn apply<F>(f: F) where
        F: FnOnce() {
        f();
    };

    fn apply_to_3<F>(f: F) -> i32 where
        F: Fn(i32) -> i32 {
        f(3)
    };

    let m = ||{
        println!("test closure!");
    };
    apply(m); // FnOnce() 으로 apply가 선언되었기 때문에 한번만 실행 가능

    let _double = |i| 2*i;
    println!("3 *2 = {}", apply_to_3(_double));

    /////////////////////////////////////////////////////////

    fn apply2<F> (f:F) where 
        F : FnOnce(){
            f();
    };

    let x3 = 7;
    let print = || println!("x3 : {}", x3);
    apply2(print);


    /////////////////////////////////////////////////////////
    // input functions

    fn call_me<F: Fn()>(f: F) {
        f();
    };
    fn function() {
        println!("I'm a function!");
    };
    let closure = || println!("I'm a closure!");

    call_me(closure);   // closure 인풋으로 사용가능
    call_me(function);  // function 인풋으로 사용가능

}
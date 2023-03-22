/*
* Rust Study
* 작성자 : 김경빈
* 참고자료    : https://sarojaba.github.io/rust-doc-korean/doc/tutorial.html   << 정리
* 볼만한 자료 : https://doc.rust-lang.org/book/title-page.html 
*               https://doc.rust-lang.org/std/fmt/ 
*/


/*
#cgo LDFLAGS: -L${SRCDIR}/..
#cgo pkg-config: ${SRCDIR}/../filcrypto.pc
#include "../filcrypto.h"
#include <stdlib.h>
#include "cgo_helpers.h"
*/
import "C"


struct Point {
    _x: f32,
    _y: f32
}

// 제어문
fn signum(x: i32) -> i32 {
    // return을 따로 선언 x
    if x < 0 { -1 }
    else if x > 0 { 1 }
    else { 0 }
}

fn main(){
    // 1. let 지역 변수
    let hi = "hi";
    let mut count = 0;  // mut로 설정

    while count < 10 {
        println!("{}",format!("count: {}", count));
        count += 1;     // mut 로 설정 했기 때문에 변경가능
    }


    //2. type casting  타입 변환
    let x: f32 = 4.8;
    let y: u32 = x  as u32; // as u32 없으면 에러
    println!("{}",y);
    assert!(y == 4u32);


    // 2-1 enum : 클래스처럼생성해서 사용하기도하고, event로 사용하기도 한다.
    // https://doc.rust-lang.org/rust-by-example/custom_types/enum.html 





    //3. 제어문
    signum(y as i32);


    //4. match
    let my_number = 10i32;
    match my_number {
        0                   => println!("zero"),
        1 | 2               => println!("one or two"),
        3..=10              => println!("three to ten"),
        _                   => println!("something else")
    }
    let mut triple = (0, -2, 3);
    match triple {
        (0, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
        (1, ..)  => println!("First is `1` and the rest doesn't matter"),
        _      => println!("It doesn't matter what they are"),
    }
    // match guard can be added to filter the arm
    let pair = (2,-3);
    let result = match pair{
        (x,y) if x==y       => (x,y),
        (x,y) if x%2 == 0   =>  (x*2, y),
        _ => (0,1)
    };
    println!("\n Pair : {:?}\n", result);


    //5. 반복문 : loop 무한루프 (while ture 와 적는것보다 선호되는 방법), while, for range, match
    let mut x = 5u32;
    loop {
        x += x - 3;
        if x % 5 == 0 { break; }
        println!("{}",x);
    }
    // 5-1 loop nesting and labels
    // break, continue 사용가능
    // 'label << 이런식으로 라벨링을 해서 break / continue 진행한다.
    'outer: loop {
        println!("Entered the outer loop");
        'inner: loop {
            println!("Entered the inner loop");
            // This would break only the inner loop
            //break;
            break 'outer;
        }
        println!("This point will never be reached");
    }
    println!("Exited the outer loop\n");

    // 5-3 loop returning
    let mut loopcnt = 0;

    let result = loop { // loop값을 result로 리턴
        loopcnt += 1;

        if loopcnt == 10 {
            break loopcnt * 2;
        }
    };
    println!("loop returning : {} \n",result);


    //5-4 for range
    for n in 1..25{
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }


    //6. 구조체
    let mut mypoint = Point { _x: 1.0, _y: 1.0 }; // mutable 설정 가능
    let origin = Point { _x: 0.0, _y: 0.0 };
    mypoint._y += 1 as f32;
    // origin._x += 2.0; // << ERROR  mutable로 설정 해주지 않았다.
    
    // 기본 match 문법 파괴
    match mypoint {
        Point { _x: 0.0, _y: yy } => { println!("struct ~ {}\n\n", yy);                     }
        Point { _x: xx,  _y: yy } => { println!("{}",format!("struct ~{}  {}\n\n",xx, yy)); }
    }


    //7. 튜플은 그냥 튜플


    //8. 함수

    /*
        fn line(a: int, b: int, x: int) -> int { a * x + b  }  
        fn oops(a: int, b: int, x: int) -> ()  { a * x + b; } //함수 마지막줄에 있는 세미콜론은 () null을 반환하는 것과 같다.
    */


    //9. box 개념
    // https://doc.rust-lang.org/rust-by-example/std/box.html 
    /*
        rust에서는 디폴트로 변수가 stack에 할당된다.
        하지만, box로 설정할 경우 heap에 할당 된다.
        Box<T>. A box is a smart pointer to a heap allocated value of type T.
    */

    let x2:Box<i32> = Box::new(12i32);
    let y2 = x2.clone();        // y는 새롭게 할당된 박스
    let z2 = x2;                // 새 메모리 할당되지 않음. x2 사용 불가.




    /* 10.  scope / freezing
    *  {} 로 블락을 생성가능하다. let으로 선언하면 block실행 동안만 할당.  
    */

    let mut _mutable_integer = 7i32;

    {
        // Shadowing by immutable `_mutable_integer`
        let _mutable_integer = _mutable_integer;    // freezing으로 mut인 변수 -> let으로

        // _mutable_integer = 50; << ERROR발생 ! let으로 mutable_integer설정했다.
    }

    // Ok! `_mutable_integer` is not frozen in this scope
    _mutable_integer = 3;




    // 11. from into   :  형변환 ex) str -> String ...
    /*  형변환을 구조체에도 사용 가능 : i32 -> Number structure 
        use std::convert::From;

        #[derive(Debug)]
        struct Number {
            value: i32,
        }

        impl From<i32> for Number {
            fn from(item: i32) -> Self {
                Number { value: item }
            }
        }

        fn main() {
            let num = Number::from(30);     // 30값을 가지는 Number 생성
            println!("My number is {:?}", num);

            let a = 5i32;
            let num2: Number = a.into();        // into  i32 -> Number
            println!("My number is {:?}", num2);
        }

    */


    // 12. parsing a String

    let parsed: i32 = "412".parse().unwrap();           // 타입을 미리 선언
    let turbo_parsed = "102".parse::<i32>().unwrap();

    let mut sum = parsed + turbo_parsed;
    println!("sum : {}",sum);


    //13. expression 

    let e:u32 = 213;

    let e1 ={
        let e_squared = e * e;
        let e_cube = e_squared * e;

        e_cube + e_squared + e // 이값으로 e1값 업데이트
    };

    let e2 = {
        2 * e;  // 세미콜론 때문에 업데이트 되지 않는다.
    };

    println!("x is {:?}", e);
    println!("y is {:?}", e1);
    println!("z is {:?}", e2);



    // 14. Option  Some None 
    // https://doc.rust-lang.org/std/option/ 
    fn divide(numerator: f64, denominator: f64) -> Option<f64> {
        if denominator == 0.0 {
            None
        } else {
            Some(numerator / denominator)
        }
    }
    let result = divide(2.0, 3.0);
    match result {
        // The division was valid
        Some(x) => println!("Result: {}", x),
        // The division was invalid
        None    => println!("Cannot divide by 0"),
    }

    let optional = None;
    check_optional(optional);

    let optional = Some(Box::new(9000i32));
    check_optional(optional);

    fn check_optional(optional: Option<Box<i32>>) {
        match optional {
            Some(p) => println!("has value {}", p),
            None => println!("has no value"),
        }
    }



    // 15. closures  : 파이썬 람다식 같음.
    let mut integer1 = 10i32;
    let plusplsu = |i:i32| -> i32 {i+1};
    println!("\nclosures plusplus : {}", plusplsu(integer1));

    /*
        Fn(), FnMut(), FnOnce() ... 존재
        Fn: the closure uses the captured value by reference (&T)
        FnMut: the closure uses the captured value by mutable reference (&mut T)
        FnOnce: the closure uses the captured value by value (T)
    */
    fn do_twice<F>(mut func: F)
        where F: FnMut()
    {
        func();
        func();
    }

    fn apply1<F>(f:F) where F:FnOnce(){
        f();
    }
    fn apply2<F:FnOnce()> (f:F){
        f();
    }
    apply1(|| println!("apply1 hello world!"));
    do_twice(|| println!("do tiwce !"));


    // 16. mapping 
    let mut myVec= Vec::new();
    myVec = (0i32..=10i32).map(|i:i32|{
        println!("asdf {}",i);  
        i*29+1
    }).collect();
    println!("myVec (mapping test) : {:?}",myVec);
}   




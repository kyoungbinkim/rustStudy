/*
* Rust Study
* 작성자 : 김경빈
* 참고자료    : https://sarojaba.github.io/rust-doc-korean/doc/tutorial.html   << 정리
* 볼만한 자료 : https://doc.rust-lang.org/book/title-page.html 
*               https://doc.rust-lang.org/std/fmt/ 
*/
use std::fmt;
use std::convert::From;

struct Matrix(f32, f32, f32, f32);

// display
impl fmt::Display for Matrix{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "({} {})\n({} {})", self.0, self.1,self.2, self.3)
    }
}


fn transpose(m : Matrix) -> Matrix{
    let r = Matrix(m.3, m.2, m.1, m.0);
    r
}

fn main(){
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    // println!("{:?}", matrix);
    println!("Matrix:\n{}", matrix);
    println!("Transpose:\n{}", transpose(matrix));



    (0..=123).map(|i| {
        let j:u32 = i+1;
    }).collect();

    println!("{}", result);
        
}
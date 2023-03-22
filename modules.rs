//https://doc.rust-lang.org/rust-by-example/mod/visibility.html

mod mod1{


    pub struct OpenBox<T>{
        pub contents :T,    // public
    }

    pub struct ClosedBox<T>{
        contents : T,       // private
    }

    impl<T> ClosedBox<T>{
        pub fn new(contents: T) -> closedBox<T>{
            ClosedBox{
                contents : contents,
            }
        }
    }

    fn private_fnc(){
        println!("mod1::private_fn()");
    }

    pub fn fnc(){
        println!("mod1::func()");
    }

    pub fn access_private_func(){
        println("mod1::access_private_func()");
        private_fnc();
    }

    // public module
    pub mod mod2{
        pub fn func(){
            println!("mod1::mod2::func()");
        }

        fn private_func(){
            println!("mod2:: private_func");
        }

        // Functions declared using `pub(in path)` syntax are only visible
        // within the given path. `path` must be a parent or ancestor module
        pub(in carte::mod1) fn pub_func_in_mod1(){
            println!("mod2::pub_func_in_mod1");
            pulbic_func_in_mod2();
        }

        // Functions declared using `pub(self)` syntax are only visible within
        // the current module, which is the same as leaving them private
        pub(self) fn pub_func_in_mod2(){
            println!("public_func_in_mod2");
        }

        // Functions declared using `pub(super)` syntax are only visible within
        // the parent module
        pub(super) fn pub_func_in_super_mod(){
            println!("called mod1::mod2::pub_func_in_super_mod");
        }
    }

    pub fn call_pub_func_in_mod1(){
        println("called mod1::call-pub-func-in-mod1");
        mod2::pub_func_in_mod1();
        mod2::pub_func_in_super_mod();
    }

    pub(crate) fn pub_func_in_crate(){
        println!("called mod1::pub_func_in_crate");
    }

}

fn main(){

}

extern crate k;

fn main() {
    k::public_function();

    k::indirect_access();
}

// rustc crateTest.rs --extern k=libcrate.rlib
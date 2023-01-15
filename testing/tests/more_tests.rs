// Integration tests file
use testing::{splish,sploosh};

#[test]
fn an_integration_test(){
    assert_eq!(sploosh(splish(-1,0),splish(1,1),splish(3,2)),4);
}

// target\debug\deps\more_tests-9a4cb27b08d1c236.exe is the compiler file
//  meaning that 9a4cb27b08d1c236 is the code's binary file pointer

/* Benchmark lesson

The lesson talks about making benchmarks on Rust. Using Criterion - look at the documentation for more info
You run the Criterion package that you'll have to create a dependency from the 
Cargo.toml file. 


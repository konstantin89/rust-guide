
// We declare that we use say_hello from this projct
use hello_world::say_hello;

// Example function
fn my_add(x: i32, y: i32) -> i32{
    return x+y;
}

fn compound_types_example(){
    // tuples are limited to 12 members.
    let my_tuple: (i32, u32, bool) = (6, 5, false);

    // accessing tuple members
    let by_bool: bool = my_tuple.2;

    // declaring arrays. Arrays are limited to the size of 32.
    let my_array: [u8; 4] = [5, 3, 6, 2];
}

fn control_flow_example(){
    let my_num = if true {5} else {8};

    // loop iterating over collection
    for c in ['a', 'r', 't'].iter(){
        println!("{}", c);
    }

    // loop iterating oer range
    for num in 0..5{
        println!("{}", num);
    }
}

fn reference_func(s: &String){
    println!("Got mutable reference to string: {}", s);
}

fn ownership_flow_example(){
    let s1: String = String::from("my string");

    // here the value of s1 MOVES to s2! s1 is invalid after this line
    let s2: String = s1;

    println!("{}", s2);

    // This will cause an compilation error!
    // s1 value has beed MOVED to s2.
    //println!("{}", s1);

    // Here we clone s2 to s3. Both are valid after this operation
    let s3: String = s2.clone();
    println!("{}", s2);
    println!("{}", s3);

    reference_func(&s3);
}


fn main() {

    // variables are immutable by default
    let _my_number = 8;
    let _my_int: i32 = 24;

    let mut _mutable_variable = 64;
    _mutable_variable = 32;

    // const variables
    const MY_CONST_VARIABLE: i64 = 1024;

    let add_result: i32 = my_add(5, 6);

    println!("Hello, world!");

    // Function from other source file
    say_hello();

    compound_types_example();

    control_flow_example();

    ownership_flow_example();
}

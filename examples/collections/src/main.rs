use std::collections::HashMap;

fn vector_example(){
    let mut my_vec: Vec<u32> = Vec::new();
    my_vec.push(2);
    my_vec.push(5);
    my_vec.push(6);
    my_vec.push(7);

    for num in my_vec{
        println!("{}", num);
    }

    // macro that allows initiating vector with known values
    let mut my_vec2 = vec![1,5,7,9];
}

fn hash_map_example(){
    let mut map: HashMap<u32, String> = HashMap::new();

    map.insert(2, String::from("two is great!"));
}

fn main() {
    vector_example();
    hash_map_example();
}

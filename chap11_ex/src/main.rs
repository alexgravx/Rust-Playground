use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let (median, mode) = integer_list_operations(vec![1,2,3,4,4,4,56,32,100,100,120,140,150]);
    println!("Median in list {median}");
    println!("Mode in list {mode}");
}

fn integer_list_operations(mut list_integers: Vec<u8>) -> (u8, u8) {
    list_integers.sort();
    let median = *(&list_integers[list_integers.len()/2]);

    let mut map: HashMap<u8, u8> = HashMap::new();
    for value in list_integers {
        let value_count = map.entry(value).or_insert(0);
        *value_count += 1;
    }
    println!("{map:?}");
    let mode = map.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().0;

    (median, *mode)
}
use std::io;

fn main() {
    fahreneit_to_celsius();
    fibonnaci();
}

fn fahreneit_to_celsius() {
    println!("Convert Fahreneit to Celsius. Enter temperature in F:");

    let mut temp = String::new();

    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");

    let temp: f32 = temp.trim().parse().expect("Please enter a correct temperature");
    let converted_temp = (temp - 32.0)*(5.0/9.0);

    println!("The temperature in Celsius is {}", converted_temp);
}

fn fibonnaci() {
    println!("Enter an index for fibonnaci calculus");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    let index: usize = index.trim().parse().expect("Please enter a correct index");

    if index == 0 || index == 1 {
        println!("The result is {}", 1);
    } else {
        let mut v: Vec<u128> = vec![1,1];
        for i in 2..index {
            v.push(&v[i-1]+&v[i-2])
        };
        println!("The result is {}", &v[v.len() - 1]);
    }
}

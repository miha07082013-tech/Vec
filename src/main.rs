use std::io;

fn main() {
    println!("===Hello VEC===");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error");

        let a: i32 = input.trim().parse().expect("Error");

        let mut v = Vec::new();
        v.push(vec![1; a.try_into().unwrap()]);

        println!("Vector - {:?}", v);
    }
}

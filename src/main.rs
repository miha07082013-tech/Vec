use std::io;

fn main() {
    println!("===Hello VEC===");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error");

        let a: i32 = input.trim().parse().expect("Error");

        let mut v: Vec<Vec<i32>> = Vec::new();
        
        let new_vec: Vec<i32> = (1..=a).collect();

        v.push(new_vec);

        println!("{:?}", v);
    }
}

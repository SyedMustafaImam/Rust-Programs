use std::io;

fn main() {
 
    println!("-------------------------------------User Value--------------------------------");

    let mut name =String::new();

    println!("Your name : ",);

    io::stdin().read_line(&mut name).expect("error");


    println!("Your name is {}",name);


}

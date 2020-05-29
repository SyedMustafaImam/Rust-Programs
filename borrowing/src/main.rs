fn main() {
    println!("\n\n\n-------------------------------Borrowing----------------------------------\n\n\n");

    let my_age = 18;
    let age_add = &my_age; // Pointer to my_age


    println!("{}", my_age);

    println!("The address of {:p}", age_add);

    println!("\n\n\n-------------------------------Pointing in Funciton----------------------------------\n\n\n");

    let my_name = String::from("SyedMustafaImam");

    let str_len = get_length(&my_name);// Here we borrowed my_name using address pointing; 
                                        // But it dose not drop its owner ship as is not the owner

    println!("The length of '{}' is '{}'", my_name,str_len);


    println!("\n\n\n-------------------------------Mutable Reference----------------------------------\n\n\n");

    let mut s = String::from("Hello");
    
    println!("My Initial String : {}", s);
    change_str(&mut s);

    println!("\n\n\n-------------------------------Data Race----------------------------------\n\n\n");

    let mut var = String::from("Hello");
    {
        let a = &mut var;
        
        println!("{}", a);

    }

    {
        let  b = &mut var;
        b.push_str(" World");
        print!("{}", b);

    }

    println!("\n\n\n-------------------------------Data Race 2----------------------------------\n\n\n");

    let mut numner = 5;
    let nu1 = &numner;
    
    println!("{}",nu1 );

    let result = numner+nu1;

    let nu2 = &mut numner;
    
    println!("Your Result is {}", result);


    println!("\n\n\n-------------------------------END----------------------------------\n\n\n");

}


fn get_length(s:&String) -> usize {


s.len()

}

fn change_str (x: &mut String){
 println!("\nAfter Passing Mutable reference :");
x.push_str(" World");
println!("{}", x);
    
}
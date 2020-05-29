fn main() {
// -------------------------------4.1 Ownership------------------------------------------------


    println!("\n\n\n---------------------String------------------\n\n\n");

    // String Type 
    // It is Stored on heap and it is dynamicly stored.

    let mut a = String :: from("Syed Mustafa");//allocate
    let mut name = a; // This is also called shallow copying 

    // the owner ship is transferd to Name
     
     name.push_str(" Imam");
     
     
        
    println!("{}" ,name);


// -------------------------------Cloning------------------------------------------------
//  This is used for copying data ftom the heap
println!("\n\n\n--------------------Cloning----------------------\n\n\n");

let s1 = String:: from("hello");
let mut s2 = s1.clone(); // Here we are getting the deep copy; Where s1 and s2 are seperate copies;

s2.push_str(" World");

println!("S1 : {}" ,s1);

println!("S2 : {}" ,s2);

println!("\n\n\n-------------------Funciton Move Ownership----------------------\n\n\n");

takes_ownership(s2);// s2 is moved to this function

// println!("After taking ownership --> S2 : {}" ,s2);
    
println!("\n\n\n--------------------Funciton Copy Ownership----------------------\n\n\n");

let num:i32 = 26;
makes_copy(num);
println!("num : {}", num); // Still can be used becaues it is copied


println!("\n\n\n--------------------Funciton Return Ownership----------------------\n\n\n");

 let  me = String::from("Mustafa");
let name = takas_and_gives_back(me);
let slogan = gives_ownership();

println!("We says : {} {} ",name,slogan);

println!("\n\n\n--------------------Funciton Return Multiple Values----------------------\n\n\n");


let my_name = String::from("Syed Mustafa Imam");
let (lenght_of_str, my_str) = get_length(my_name);

println!("Length of '{}'  is '{}'" , my_str, lenght_of_str );


println!("\n\n-------------------------------END-------------------------------\n\n\n");
}
// a variable is droped here means deallocated automatically




fn takes_ownership (x:String) {
    println!("Ownership transfered from s2 to function");

    println!("S2 -> X: {}", x);
}

fn makes_copy(x:i32){

    println!("Copied from num -> x : {}", x);
}

fn gives_ownership() -> String {

    let s = String::from("El Genio");

    s
}

fn takas_and_gives_back(x:String)-> String{

x // is moved out to the calling function

}

fn get_length (strings : String) -> (usize,String) {

    (strings.len(),strings)
}
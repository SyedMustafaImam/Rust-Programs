#[derive(Debug)]

struct Student {
    name : String,
    grade: String,
    age: u32,
    percentage : f32
}


fn main() {
println!("\n\n\n---------------------------------Assignment 3---------------------------------\n\n\n");

let student_mustafa = Student::student1();

println!("{:#?}",Student::student1());


student_mustafa.student_per();


println!("\n\n\n--------------------------------------END--------------------------------------\n\n\n");
}

impl Student {


    fn student1 () -> Student{
       let s1 = Student {
            name:String::from("Syed Mustafa Imam"),
            grade:String::from("A+"),
            age:21,
            percentage:95.45
        };

        s1
    } 


    fn student_per(&self){

        println!("\n\nSudent '{}' Percentage = {}",self.name,self.percentage);
    }
}
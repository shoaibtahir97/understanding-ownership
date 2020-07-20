fn main() {
    /*student is not valid as it is not declared
        let student  =  "Shoaib";   student is valid from this point
            do stuff with student
    }                              scope is over and student is not valid
    */
    let student = String::from("Ramsha"); //String type can be mutated
    let mut developer = String::from("Shoaib");
    developer.push_str(" loves Rust");
    let openSourceContributer = developer.clone();
    println!("{} {}", developer, openSourceContributer);

// let s = String::from("Hello");

}

fn takes_candy(string_type: String){
    println!("{}",string_type);
}

fn makes_money()





use self::{name_helpers::get_full_name, private_fns::get_age};



pub mod name_helpers {

    pub fn get_full_name(first_name: &str, last_name:&str) -> String {
    
       let full_name: String  = format!("{} {}", first_name, last_name);
    
       return full_name;
    }
}

pub mod private_fns {

    pub fn get_age(age: u8) -> u8 {

        return age;

    }
}

pub fn give_info(first_name :&str, last_name : &str, age: u8) {
    let info = format!("{} ", get_full_name(first_name, last_name));
    if get_age(age) > 40 {

        println!("INFO FOR YOu: {}", info);
    }
    else {
        
        println!("No info for you ");
    }
}

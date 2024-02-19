

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

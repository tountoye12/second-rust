

pub fn test_match() {

    println!("Test matches");

    let age:u32= 45;
    match age {

        50 => {
            println!("YOu have to wait 5 more year");
        },

        _ => {
            println!("You good to go");
        }
        
        
    }
}
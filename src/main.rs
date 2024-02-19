

mod functions;

mod closures;

mod test_matches;

mod generics;

fn main() {

    // let full_nale  = 
    //     functions::name_helpers::get_full_name("Diallo", "Mamadou");

    // let age = functions::private_fns::get_age(34);

    // println!("Full name: {} ==> age: {}", full_nale, age);

    // functions::give_info("Diallo","Mad", 30 );

    // closures::test_closures();

    //test_matches::test_match();
    let list = vec![10,2,3,4,30];
    let list_chars = vec!['a','b', 'c', 'd'];


    let largest = functions::largest(&list);
    let largest_char = functions::largest_char(&list_chars);
    // let largest = generics::get_largest(&list);
    // let largest_char = generics::get_largest(&list_chars);

    println!("Largest is:  {:?}", largest);
    println!("Largest char is:  {:?}", largest_char);

}

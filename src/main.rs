

mod functions;

mod closures;

fn main() {

    let full_nale  = 
        functions::name_helpers::get_full_name("Diallo", "Mamadou");

    let age = functions::private_fns::get_age(34);

    println!("Full name: {} ==> age: {}", full_nale, age);

    functions::give_info("Diallo","Mad", 30 );

    closures::test_closures();

}

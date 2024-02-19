
pub fn test_closures() {
    let print_hello  = |name: &str| println!("Test closures {}", name);
    print_hello("Diallo");

    let add  = |x ,y| {

        println!("x = {} and  y = {}", x ,y);

        x + y
    };  
    let result  = add(10, 5);

    println!("x + y = {}", result);


}

// when borrowing the variable and the closure must be mutable both
struct  Person {
    f_name : String,
    l_name: String
}
pub fn test_borrow() {

    let mut p1 = Person {
        f_name: "diallo".to_string(),
        l_name: "Mamadou".to_string()
    };

    let mut change_name = |new_name:String| p1.f_name = new_name;

    change_name("Adama".to_string());
}
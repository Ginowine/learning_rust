fn main() {
    let mut s = String::from("Hello");
    change(&mut s);

    println!("{}", s);
    change_fail();
}

fn change(some_string: &mut String){
    some_string.push_str(", world");
}

fn change_fail(){
    let mut s = String::from("Hello");

    {
        let r1 = &mut s;
        println!("{}", r1);

    }
    
    let r2 = &mut s;

    println!("{}", r2);

}

fn main() {

    let mut s = String::from("hello");
    s.push_str(", world");

    println!("{}", s);
    string_test();

    let x = 5;

    takes_ownership(s);

    // println!("{}", s); trying to do this will cause an error bcos the scope of s has been moved
    makes_copy(x);

}

fn string_test(){
    let s = String::from("hello world");
    let s2 = s;
    println!("{}", s2);
}

fn takes_ownership(some_string: String){
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32){
    println!("{}", some_integer);
}

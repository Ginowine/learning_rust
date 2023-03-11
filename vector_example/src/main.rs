

fn main (){
    // This is one way to add values to a vector
// let v = vec![1,2,3,4,5];

// This is another way to add values to vector using push method
let mut v = Vec::new();

v.push(1);
v.push(2);
v.push(3);
v.push(4);
v.push(5);

let third: &i32 = &v[2];
println!("The third element is {third}");

let third:Option<&i32> = v.get(2);
match third {
    Some(third) => println!("The third element is {third}"),
    None => println!("There is no third element."),
}
}
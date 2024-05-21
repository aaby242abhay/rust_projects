// pub enum Option<T>{
//     None,
//     Some(T),
// };

fn find_first_a(s : &String) -> Option<usize>{
    for (index, char) in s.chars().enumerate(){
        if char == 'a'{
            return Option :: Some(index);
        }
    }
    Option :: None
}
fn main(){
    let my_string = String :: from("Hello world");
    let res = find_first_a(&my_string);

    match res{
        Some(index) => println!("The index of first 'a' is : {}", index),
        None => println!("There is no 'a' in the string"),
    }

}
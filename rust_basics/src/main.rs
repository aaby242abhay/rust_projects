struct User{
    active : bool,
    username : String,
    email : String,
    sign_in_count : u64
}
fn main(){
    println!("Hello rustains");

    let user1 = User{
        active : true,
        username : String::from("someusername@123"),
        email : String::from("someemail@123"),
        sign_in_count : 1
    };

    println!("User1 username is : {:?}", user1.username);
}
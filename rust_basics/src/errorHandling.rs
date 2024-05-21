use std :: fs;

enum Result<T,E>{
    Ok(T),
    Err(E),
}

fn main(){
    let greeting = fs :: read_to_string("hello.txt");

    match greeting{
        Ok(content) =>{
            println!("The content of the file is : {}", content);
        }
        Err(error) =>{
            println!("There was an error reading the file : {}", error);
        }
    }

}
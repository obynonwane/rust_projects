
use greeter::greet_user;

fn main() {
    let user_name: Result<String, anyhow::Error> = greet_user();
    println!("hello, {}!", user_name.unwrap().trim());
}



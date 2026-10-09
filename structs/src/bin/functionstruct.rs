#[derive(Debug)]
struct user {
    active : bool,
    username : String,
    email: String,
    sign_in_count : u64,
}
fn build_user(active:bool,username:String,email:String,sign_in_count:u64)-> user
{
user
{
    active: active,
    username : username,
    email : email,
    sign_in_count: sign_in_count,
}
}
fn main()
{
    let user1 = build_user(
        false,
        String::from("kaushik"),
        String::from("kaushik@gmail.com"),
        2,
    );
    println!("{:#?}",user1);
        println!("{}",user1.active);
            println!("{}",user1.username);
                println!("{}",user1.email);
                println!("{}",user1.sign_in_count);
            }

struct user 
{
    active : bool ,
    username : String,
    email : String,
    sign_in_count: u64,
}
fn main()
{
let user1 = user
{
    active : true,
    username: String ::from("someusername123"),
    email: String::from("someone@example.com"),
    sign_in_count: 1 ,
};
let user2 = user
{
 email:String::from("newexample@gmail.com"),
 ..user1
};
println!("active: {}",user2.active);
println!("username: {}",user2.username);
println!("email: {}",user2.email);
println!("sign-in count: {}",user2.sign_in_count);




}

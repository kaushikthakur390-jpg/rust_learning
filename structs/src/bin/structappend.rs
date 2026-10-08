struct employee
{
    name : String ,
    empID : u64,
    email : String,
    salary : f64,
}
fn main ()
{
    let mut emp1 = employee
    {
        name : String :: from("kaushik"),
        empID : 452,
        email : String :: from("kaushik@gmail.com"),
        salary : 60000.12,
    };
    emp1.email = String :: from("kaushikthakur390@gmail.com");
    println!("email : {}",emp1.email);
}
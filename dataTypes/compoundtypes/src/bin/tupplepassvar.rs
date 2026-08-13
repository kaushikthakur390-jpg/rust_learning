fn main()
{
    let tup : (u32,i32,f64) = (500,200,6.8);
    let (x,y,z) = tup;//an other way is that we can pass the values of tupple to variables by destructuring 
    println!("the values of tupple are {x},{y},{z}");
}
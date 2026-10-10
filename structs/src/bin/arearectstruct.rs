struct Rectangle 
{
    width : u32,
    height : u32,
}
fn main ()
{
    let rect1 = Rectangle
    {
        width:50,
        height:20, 
    };
    println!("area of rectangle is {} pixels",area_rectangle(&rect1));
}
fn area_rectangle(rectangle : &Rectangle)->u32
{
    rectangle.width * rectangle.height
}
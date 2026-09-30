fn main()
{
    let mut counter = 0 ;
    'counting : loop {
        println!("count = {counter}");
        let mut remaining  = 10;
        loop{
            if remaining == 9 
            {break}
         println!("remaining:{remaining}");
         if counter == 2 
         {
            break 'counting ; 
         }
         remaining -=1;   
        }
        counter += 1;
    }
    println!("end count = {counter}")
}
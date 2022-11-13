fn main() 
//Rust progam to find modular multiplicative
//inverse using Extended Euclidean algorithm.
   
{
   print!("\n Test Modular multiplicative inverse\n");
   //Test function
   invert_modulo(7, 34);
   invert_modulo(9, 28);
   invert_modulo(28, 9);
  
}
 //Calculates the modular multiplicative inverse of an integer
fn invert_modulo(a: i32, m: i32)

{
    let num: i32 = a % m;
    //Loop controlling variable
    let mut i: i32 = 1;
    //result variable
    let mut result: i32 = -1;
    //Display given data information
    print!("\n Number [a] : {}", a);
    print!("\n Modular [m] : {}", m); 

//Runs the conditional expression before running the loop body
    while i < m && result == -1
    {
        if (num * i) % m == 1 
        {
            //When get result
            result = i;
        }
        i += 1;
    }
    if result == -1
    {
        print!("\n Inverse are not found\n");
    }
    else
    {
        print!("\n Result {}\n", result);
    }

}

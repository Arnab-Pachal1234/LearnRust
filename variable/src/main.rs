


fn main() {
    println!("Hello, world!");
    let  age  = 24; 
    println!("the value of age is {age}");
    
    //mutable 
    let s = "Hello".to_string();
    println!("{}",s);
  
   //const 
   const PI : f64 = 3.14;
   println!("{}",PI);
    
    //for constant we should always explict mention type 
    //constant can be defined in any scope but let always have to defined in the loca scope
    //constant value can be always defined in the compile time not in run time
    //convention for constant all uppercase letter

    /*shadowing */
    
    let apples :i32 = 10;
    let apples :i64 = 20;
    println!("apples {apples}");

    let apples : i32 =100;
    let apples :i32 = apples + 200;
    println!("{}",apples)

}

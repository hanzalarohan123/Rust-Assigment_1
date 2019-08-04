//tiny.cc/IOTGS1
//Q1
// fn main()
// {
//     println!("hello");
//     //it wis not compiling due to missing " ! " after "println!"
// }


// //Q2
// fn main() {
// let chocolate1 = 10;
// let chocolate2 = 10;
// let total: u32 = chocolate1 + chocolate2; //constant cannot be assiged to variable
// println!("The sum of x and y is:{} ",total); // {} was misign 
// }

// //Q3
// fn main() {
// let x = 2.5;
// let y = 3;
// let z :f32= x  * y as f32;// multiplying two diffetn data types wthout specifying isnt allowed in rust so we have to first specify the types and convert then we can perforn any action
// println!("{}",z);
// }


// //Q4
// fn main() 
// {
// let radius = 6.0;
// let perimeter:f32; //just change the type form i32 to f32
// perimeter = 2.0*3.14*radius; // this will calculate to floating points
// println!("Perimeter of the Circle = {} inches", perimeter);
// }

// //Q5
// fn main(){
// let x = "haris";// mut was there it means it was changeable and occupyong shadow down ther of x.len
// println!("{}",x);
// let x = x.len();// let was missing due to which it was giving error
// println!("{}",x) ;
// }

// //Q6
// fn main() {
// let mut x = 3;//mut helps us to reuse variable
// println!("Number {}", x);
// x = 5;
// println!("Number {}", x);
// }

//Q7
// fn main() {
// // Short-circuiting boolean logic
// println!("true AND false is {}", true && false);
// println!("true OR false is {}", true || false);
// println!("NOT true is {}", !true);
// }

// //Q8
// fn main() 
// {
// let interest:i32 = 8;//i32 is for integers 32 bit 
// println!("interest is {}",interest);
// }

// //Q9
// fn main() 
// {
// let long_lived_binding = 1;
// // This is a block, and has a smaller scope than the main function
// {
// // This binding only exists in this block
// let short_lived_binding = 2;
// println!("inner short: {}", short_lived_binding);
// println!("inner short: {}", long_lived_binding);
// }
// // Error! `short_lived_binding` doesn't exist in this scope
// //println!("outer short: {}", short_lived_binding);//varaible was local so its scope was not allowed outside if you want to print that so you need to get the variable initialize outside the block as local variable
// // FIXME ^ Comment out this line
//}
// //Q10
// fn main() {
// let a_binding;
// {
// let x = 2;
// // Initialize the binding
// a_binding = x * x;
// }
// println!("a binding: {}", a_binding);//varaible name should be written completely
// }

//PART 2
//Q1 Percentage Calculation
//  fn main()
//  {
//      let x  = 95;
//      let y  = 100;
//      let percentage : f32 =(x as f32/ y as f32) * y as f32;
//      println!("Percentage is {}",percentage);
//  }


//Q2


// fn main(){
//    let mut line = String::new();
//    println!("Enter your name :");
//    let b1 = std::io::stdin().read_line(&mut line).unwrap();
//    println!("Hello , {}", line);
//    println!("no of bytes read , {}", b1);
// }

//Q3
// fn main()
// {
//     let name = "Muhammad Hanzala";
//     let number : i64 = 03441299589;
//     let age : i32 = 21;
//     println!("{}\n{}\n{}",name,age,number);
// }

//Q4
// fn main()
// {
// let  _a = 125;
// let  _b = 12345;
// let _ax : i64 = 1234567890;
// let _s : i32 = 4043;
// let __x : f32 = 2.13459;
// let _dx : f32 = 1.1415927;
// let _c : char = 'W';
// let _x : u32 = 2541567890;

//println!("a + c is :{}",_a + _c as i32);
//println!("x + c is :{}",__x + _c as i32 as f32);
//println!("dx + x is :{}",_dx + _x as f32);
//println!("a + x is : {}",_a + _x);
//println!("s + b is :{}",_s + _b);
//println!("ax +  b is :{}",_ax + _b);
//println!("s + c : {}",_s + _c as i32);
//println!("ax + c is :{}",_ax + _c as i64);
//println!("ax + ux : {}",_ax + _x as i64);

//}

//Q5
// fn main()
// {
//     let days = 1329;
//     let year : i32 = days/365;
//     let rem =days%365;//234 rem
//     let weeeksrem=rem/7;

//     println!(" years are :{}",year);
//     println!(" Weeks are :{}",weeeksrem);
    

// }
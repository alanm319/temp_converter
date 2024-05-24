use std::io;
fn main() {

//  println!("Enter 0 for Fehrenheit to Celsius, or 1 for Celsius to Fehrenheit");
//  let mut flag = String::new();
//  io::stdin()
//      .read_line(&mut flag)
 //     .expect("Failed to read line");
  
 // let flag: i32 = flag.trim().parse().expect("Please type a numer!");
// this is a comment
 
  
  println!("Enter a temperature in Fehrenheit:"); 
  let mut temp = String::new();
  io::stdin()
      .read_line(&mut temp)
      .expect("Failed to read line");
  
  let temp: f64 = temp.trim().parse().expect("Please type a numer!");
  let converted_temp = temp_conversion(temp);

  println!("{temp} in celsius is {converted_temp}");



}

fn temp_conversion(temp: f64) -> f64{
    
    (temp-32.0) * (5.0/9.0)
}

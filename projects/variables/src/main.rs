fn main() {
  let x = 5;
  let x = x + 1;

  {
    let x = x * 2;
    println!("the value of x in the inner scope is: {x}")
  }
  
  println!("the value of x is: {x}")
}
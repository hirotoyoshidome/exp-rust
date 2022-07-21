fn main() {
  println!("Hello");

  let x: i32 = 10;
  let y: i32 = 5;

  println!("{}", x+y);


  if x > 5 {
    println!("high");
  } else {
    println!("low");
  }

  for i in 0..10 {
    println!("{}", i);
  }
}


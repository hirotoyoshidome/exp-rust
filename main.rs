fn main() {
  // 出力 !が必要
  println!("Hello");

  // 変数
  let x: i32 = 10;
  let y: i32 = 5;

  println!("{}", x+y);

  // if文
  if x > 5 {
    println!("high");
  } else {
    println!("low");
  }

  // for文
  for i in 0..10 {
    println!("{}", i);
  }
}

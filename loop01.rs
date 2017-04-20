fn main() { 
 let mut x:i32 = 1;
 loop{
  println!("Loop: {}",x);
  if x>=10 {
   break;
  }
  x += 1;
 }
}

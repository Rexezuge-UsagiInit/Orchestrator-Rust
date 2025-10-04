use std::env;

pub fn set(key: &str, value: &str){
  unsafe{
    env::set_var(key, value);
  }
  println!("Configured Environment Variable: {} = {}", key, value);
}

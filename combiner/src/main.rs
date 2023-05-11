fn main() {
    println!("This is going to be a combiner code written in Rust!");
  }
  
  fn get_nth_arg(_n: usize) {
  }
  
  #[cfg(test)]
  mod tests {
    use crate::get_nth_arg;
  
    #[test]
    fn get_nth_arg_takes_usize() {
      let _ = get_nth_arg(0usize);
      assert!(true, "Your get_nth_arg function should take a usize argument.");
    }
  
    #[test]
    fn regex_crate_is_installed() {
      let string_to_test = "println!(\"Hello, world!\")".to_string();
      assert!(reg_with_con(r"Hello, world!", string_to_test));
    }
  
    fn reg_with_con(regex: &str, file_contents: String) -> bool {
      use regex::Regex;
  
      Regex::new(regex).unwrap().is_match(&file_contents)
    }
  }
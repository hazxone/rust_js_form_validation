fn is_minimum(text : &str, max_length : usize) -> bool {
  if text.len() < max_length
  {
    return false;
  }

  return true;
}

fn char_in_string(s: &str, c: char) -> bool {
    for ch in s.chars() {
        if ch == c {
            return true;
        }
    }
    false
}

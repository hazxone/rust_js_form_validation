fn is_minimum(text : &str, min_length : usize) -> bool {
  if text.len() < min_length
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

fn main() {
    if is_minimum("foobar", 4)
    {
      println!("Yay");
    }
    else
    {
      println!("Nay");
    }

    if char_in_string("mee@mail.com", '@')
    {
      println!("Yay");
    }
    else
    {
      println!("Nay");
    }
    
    let email = "example@example.com";
    let is_valid = is_valid_email(email);
    println!("Is {} a valid email? {}", email, is_valid);
}

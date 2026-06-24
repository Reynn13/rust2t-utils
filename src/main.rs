

use rust2t_utils::{io::InputUtil, auth::{Auth, SecureTextDesc, SecureTextFreq}};

fn main() -> std::io::Result<()> {
   println!("Hello, world!");
   let stdin = std::io::stdin();
   let stdout = std::io::stdout();

   let mut iou = InputUtil::new(stdin.lock(), stdout.lock());

    let name = iou.read_line_new("Your name? ")?.trim().to_string();
    let age: u32 = iou.read_parse_validate_retry("Your age(1-120): ", "Invalid age, try again!", "Age must between 1-120", |x| *x >= 1 && *x <= 120);

    println!("Your name: {} | Your age: {}", name, age);
    let mut auth = Auth::new(iou);
    let password = auth.read_secure_custom("Your password: ", SecureTextDesc::new("You can't see it!", SecureTextFreq::Once))?;

    println!("Password: {}", password);
    Ok(())
}

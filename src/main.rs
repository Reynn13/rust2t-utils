

use rust2t_utils::{io::InputUtil, auth::{AuthUtil, SecureTextDesc, SecureTextFreq}};

fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();

    let mut iou = InputUtil::new(stdin.lock(), stdout.lock());

    let name = iou.read_line_new("Your name? ")?;
    let age: u32 = iou.read_parse_validate_retry("Your age(1-120): ", "Invalid age, try again!", "Age must between 1-120", |x| *x >= 1 && *x <= 120);

    println!("Your name: {} | Your age: {}", name, age);
    let mut auth = AuthUtil::new(iou);
    let password = auth.read_confirmed(|auth| auth.read_secure("Your password? "))?;

    println!("Password: {:?}", password);
    Ok(())
}

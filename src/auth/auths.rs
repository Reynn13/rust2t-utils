
use crate::io::InputUtil;
use std::io::{self, BufRead, Write};

pub struct Auth<R: BufRead, W: Write> {
    iou: InputUtil<R, W>
}

pub struct SecureTextDesc {
    secure_text: String,
    frequency: SecureTextFreq
}

impl SecureTextDesc {
    pub fn new(secure_text: impl ToString, frequency: SecureTextFreq) -> Self {
        Self { secure_text: secure_text.to_string(), frequency }
    }
}

#[derive(PartialEq)]
pub enum SecureTextFreq {
    Once,
    Everytime
}

impl<R: BufRead, W: Write> Auth<R, W> {
    pub fn new(iou: InputUtil<R, W>) -> Self {
        Self { iou }
    }
    
    pub fn read_secure_custom(&mut self, prompt: &str, desc: SecureTextDesc) -> io::Result<String> {
        write!(self.iou.writer, "{}", prompt)?;
        self.iou.writer.flush()?;

        let mut password = "".to_string();
        
        let _raw_guard = self.iou.raw_mode()?;

        loop {
            if desc.frequency == SecureTextFreq::Once {
                write!(self.iou.writer, "{}", &desc.secure_text)?;
                self.iou.writer.flush()?;
            }

            let byte = self.iou.read_raw_byte()?;
            
            if byte == b'\n' || byte == b'\r' {
                writeln!(self.iou.writer)?;
                break;
            } else if byte == 127 || byte == 8 {
                if !password.is_empty() {
                    password.pop();
                    if desc.frequency == SecureTextFreq::Everytime {
                        write!(self.iou.writer, "\x08 \x08")?;
                        self.iou.writer.flush()?;
                    }
                }
            } else {
                if let Ok(ch) = std::str::from_utf8(&[byte]) {
                    password.push_str(ch);
                    if desc.frequency == SecureTextFreq::Everytime {
                        write!(self.iou.writer, "*")?;
                        self.iou.writer.flush()?;
                    }
                }
            }
            if desc.frequency == SecureTextFreq::Once {
                let len = desc.secure_text.len();
                write!(self.iou.writer, "{}{}{}", "\x08".repeat(len), " ".repeat(len), "\x08".repeat(len))?;
                self.iou.writer.flush()?;
            }
        }
        Ok(password)
    }
        

    pub fn read_secure(&mut self, prompt: &str) -> io::Result<String> {
        self.read_secure_custom(prompt, SecureTextDesc::new("*", SecureTextFreq::Everytime))
    }
}

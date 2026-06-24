
use std::io::{self, BufRead, Write};
use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
pub enum InputErr {
    IoError(io::Error),
    ParseErr,
}

fn fast_trim(s: &mut String) {
    let trimmed_len = s.trim_end().len();
    let start = s.trim_start().as_ptr() as usize - s.as_ptr() as usize;

    s.truncate(start + trimmed_len);
    s.drain(0..start);
}

impl fmt::Display for InputErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InputErr::IoError(e) => write!(f, "IO Error: {}", e),
            InputErr::ParseErr => write!(f, "Invalid Input Format"),
        }
    }
}

impl From<io::Error> for InputErr {
    fn from(err: io::Error) -> InputErr {
        InputErr::IoError(err)
    }
}

pub struct RawModeGuard {
    orig_termios: libc::termios
}

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        unsafe {
            libc::tcsetattr(0, libc::TCSANOW, &self.orig_termios);
        }
    }
}

pub struct InputUtil<R: BufRead, W: Write> {
    pub(crate) reader: R,
    pub(crate) writer: W
}

impl<R: BufRead, W: Write> InputUtil<R, W> {
    pub fn new(reader: R, writer: W) -> Self    {
        Self {
            reader,
            writer
        }
    }

    pub fn read_line_append(&mut self, buffer: &mut String, prompt: &str) -> io::Result<usize> {
        write!(self.writer, "{}", prompt)?;
        self.writer.flush()?;
                
        self.reader.read_line(buffer)
    }

    pub fn read_line_reset(&mut self, buffer: &mut String, prompt: &str) -> io::Result<usize> {
        buffer.clear();
        let l = self.read_line_append(buffer, prompt)?;
        fast_trim(buffer);
        Ok(l)
    }

    pub fn read_line_new(&mut self, prompt: &str) -> io::Result<String> {
        let mut buffer = "".to_string();
        self.read_line_reset(&mut buffer, prompt)?;
        Ok(buffer)
    }

    pub fn read_parse<T>(&mut self, prompt: &str) -> Result<T, InputErr>
    where
        T: FromStr
    {
        let input = self.read_line_new(prompt)?;
        input.parse::<T>().map_err(|_| InputErr::ParseErr)
    }

    pub fn read_parse_retry<T>(&mut self, prompt: &str, err_msg: &str) -> T
    where 
        T: FromStr
    {
        loop {
            match self.read_parse::<T>(prompt)
            {
                Ok(value) => return value,
                Err(_) => {
                    writeln!(self.writer, "{}", err_msg).unwrap();
                    self.writer.flush().unwrap();
                }
            }
        }
    }
    
    pub fn read_parse_validate<T, F>(&mut self, prompt: &str, f: F) -> Result<Option<T>, InputErr> 
    where
        T: FromStr,
        F: Fn(&T) -> bool
    {
        let v: T = self.read_parse(prompt)?;
        if f(&v) {
            Ok(Some(v))
        } else {
            Ok(None)
        }
    }

    pub fn read_parse_validate_retry<T, F>(&mut self, prompt: &str, parse_err_msg: &str, val_err_msg: &str, f: F) -> T
    where
        T: FromStr,
        F: Fn(&T) -> bool,
    {
        loop {
            
            let v: T = self.read_parse_retry(prompt, parse_err_msg);
            if f(&v) {
                return v;
            }
            
            writeln!(self.writer, "{}", val_err_msg).unwrap();
            self.writer.flush().unwrap();
        }
    }

    pub fn raw_mode(&mut self) -> io::Result<RawModeGuard> {
        let stdin_fd = 0; // STDIN_FILENO

        let mut orig_termios = unsafe { std::mem::zeroed() };
        if unsafe { libc::tcgetattr(stdin_fd, &mut orig_termios) } != 0 {
            return Err(io::Error::last_os_error());
        }

        let mut raw_termios = orig_termios;
        raw_termios.c_lflag &= !(libc::ECHO | libc::ICANON);
        if unsafe { libc::tcsetattr(stdin_fd, libc::TCSANOW, &raw_termios) } != 0 {
            return Err(io::Error::last_os_error());
        }
        
        Ok(RawModeGuard { orig_termios })
    }

    pub fn read_raw_byte(&mut self) -> io::Result<u8> {
        let mut buf = [0u8; 1];
        let n = unsafe { libc::read(0, buf.as_mut_ptr() as *mut libc::c_void, 1) };
        if n <= 0 {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "EOF or Error"));
        }
        Ok(buf[0])
    }
}



use std::{fmt, fs::File, io::Write, num::ParseIntError};

#[derive(Debug)]
pub enum MyError {
    ParseError,
    IOError
}

// 러스트의 error 타입은 러스트 표준 라이브러리의 Error 타입을 구현한다
impl std::error::Error for MyError {}

/// 러스트의 Error trait는 Debug와 Display trait를 구현해아 함
/// Debug trait는 자동으로 유도되며, Display trait는 여기에서 구현
impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::ParseError => write!(f, "Parse Error"),
            MyError::IOError => write!(f, "IO Error"),
        }
    }
}

fn main() {
    // println!("{:?}", square("2"));
    let result = square("INVALID");
    match result {
        Ok(res) => println!("Result is {:?}", res),
        Err(e) => println!("Error in parsing: {:?}", e)
    };
}

fn square(val: &str) -> Result<i32, MyError> {
    let num = val.parse::<i32>().map_err(|_| MyError::ParseError)?;

    let mut f = File::open("fictionalfile.txt").map_err(|_| MyError::IOError)?;
    
    let string_to_write = format!("Square of {} is {}", num, i32::pow(num, 2));
    f.write_all(string_to_write.as_bytes()).map_err(|_| MyError::IOError)?;

    Ok(i32::pow(num, 2))
}

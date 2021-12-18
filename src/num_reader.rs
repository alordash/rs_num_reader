use num_traits::{self, Num};
use std::{
    io::{Read, Stdin},
    str::FromStr,
};

pub const DELIMITERS: &'static str = " \n,";

pub fn read_num<T>(io: &Stdin) -> Result<T, <T as FromStr>::Err>
where
    T: Num + FromStr,
{
    read_num_with_delimiters(io, DELIMITERS)
}

pub fn read_num_with_delimiters<T>(io: &Stdin, delimiters: &str) -> Result<T, <T as FromStr>::Err>
where
    T: Num + FromStr,
{
    let mut num_str = String::new();
    let mut io = io.lock();
    loop {
        let mut c = [0; 1];
        loop {
            match io.read(&mut c) {
                Ok(_) => break,
                Err(_) => (),
            }
        }
        let c = c[0] as char;
        if delimiters.contains(c) {
            if num_str.len() == 0 {
                continue;
            }
            break;
        }
        num_str.push(c);
    }
    num_str.trim().parse::<T>()
}

pub fn read_and_handle<T>(io: &Stdin) -> T
where
    T: Num + FromStr,
    <T as FromStr>::Err: std::fmt::Display,
{
    loop {
        match read_num::<T>(io) {
            Ok(v) => break v,
            Err(err) => {
                println!("Error parsing number: \"{}\"", err);
            }
        }
    }
}

pub fn read_and_handle_with_delimiters<T>(io: &Stdin, delimiters: &str) -> T
where
    T: Num + FromStr,
    <T as FromStr>::Err: std::fmt::Display,
{
    loop {
        match read_num_with_delimiters::<T>(io, delimiters) {
            Ok(v) => break v,
            Err(err) => {
                println!("Error parsing number: \"{}\"", err);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

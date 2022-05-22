#![allow(dead_code)]

use std::{fmt, str::FromStr};
struct Circle {
    radius: i32,
}

// impl fmt::Display for Circle {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "Circle of radius {}", self.radius)
//     }
// }

impl ToString for Circle {
    fn to_string(&self) -> String {
        format!("Circle of radius Ts {:?}", self.radius)
    }
}

#[test]
fn test_to_string() {
    let circle = Circle { radius: 16 };
    //实现Display免费获得一个 to_string方法
    println!("{}", circle.to_string());
}

impl FromStr for Circle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() > 5 {
            Ok(Self {
                radius: s.len() as i32,
            })
        } else {
            Err(())
        }
    }
}

#[test]
fn test_form_str() {
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println! {"Sum: {:?}", sum};

    let cc = "1000999900".parse::<Circle>().unwrap();

    println!("{}", cc.to_string())
}

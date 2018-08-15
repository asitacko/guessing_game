use std::fmt::{self, Formatter, Display};
extern crate chrono;
extern crate time;
use chrono::prelude::*;
use time::Duration;

#[derive(Debug)]
struct B {
    t3: i32,
}

#[derive(Debug)]
pub enum WeekdayBits {
    Sun = 1,
    Mon = 2,
    Tue = 4,
    Wed = 8,
    Thu = 16,
    Fri = 32,
    Sat = 64,
}

#[derive(Debug)]
struct A {
    t1: i32,
    t2: Vec<B>,
}

impl A {
    pub fn create(t1: i32, t2: Vec<B>) -> A {
        A {
            t1: t1,
            t2: t2,
        }
    }
}

//impl Display for A {
//    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
//        write!("t1: {}, t2: {}", (self.t1, self.t2));
//    }
//}

impl B {
    pub fn create(t3: i32) -> B {
        B {
            t3: t3,
        }
    }
}

fn main() {
    let mut b = vec!(B::create(2));
    let mut a = A::create(1, b);
    println!("{:?}", a);
    let mut time = Utc::now().date() + Duration::days(3);
    let dat = Utc::now().date().month();
    let dat1 = Utc::now().date().year();
    let dat2 = Utc::now().date().weekday() as i32;
    let daybits = match dat2 {
        0 => WeekdayBits::Mon,
        1 => WeekdayBits::Tue,
        _ => WeekdayBits::Wed,
    };
    println!("{}, {}, {:?}, {:?}", dat,dat1, daybits, a);
}

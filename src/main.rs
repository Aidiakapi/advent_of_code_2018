#![feature(stmt_expr_attributes, drain_filter, try_from)]

#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate serde;
extern crate bincode;
extern crate reqwest;
extern crate crypto;
#[cfg_attr(test, macro_use)]
extern crate smallvec;
extern crate num_traits;
extern crate twoway;
extern crate pathfinding;

mod error;
mod vec2;
mod mat2;
#[macro_use]
mod framework;

use std::env;
use reqwest::Client;
use crate::framework::Framework;

pub(crate) use crate::error::Error;
pub(crate) use crate::error::Result;

macro_rules! main {
    ($($days:ident),+$(,)*) => {
        $(
            mod $days;
        )+
        fn main() {
            let mut fw = Framework::new();

            $(
                {
                    crate::$days::register_day(&mut fw);
                }
            )+;

            let client = Client::new();

            let args: Vec<String> = env::args().collect();
            match args.len() {
                1 => {
                    // execute all
                    $(
                        {
                            if let Err(e) = fw.execute(&client, stringify!($days)) {
                                eprintln!("{}", e);
                                std::process::exit(-2);
                            }
                        }
                    )+;
                },
                2 => {
                    // execute specific day
                    if let Err(e) = fw.execute(&client, args[1].as_str()) {
                        eprintln!("{}", e);
                        std::process::exit(-2);
                    }
                },
                _ => {
                    eprintln!("too many arguments");
                    std::process::exit(-1);
                }
            }
        }
    };
}

main!(
    day01,
    day02,
    day03,
    day04,
    day05,
    day06,
    day07,
    day08,
    day09,
    day10,
    day11,
    day12,
    day13,
    day14,
    day15,
    day16,
    day17,
    day19,
);

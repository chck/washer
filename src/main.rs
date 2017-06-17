// #![cfg_attr(feature="clippy", feature(plugin))]
// #![cfg_attr(feature="clippy", plugin(clippy))]
extern crate regex;

use regex::Regex;

fn main() {
    let str = "09__2_233";
    let re = Regex::new(r"\d+").unwrap();
    let caps = re.captures(str).unwrap();
    println!("Hello, world!: {}", &caps[0]);
}

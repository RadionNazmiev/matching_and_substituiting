extern crate regex;
use regex::Regex;

fn change(s: &str, prog: &str, version: &str) -> String {
    let cv = Regex::new(r"Version: (?P<Version>\d+\.\d+)\n").unwrap().captures(s);
    let cp = Regex::new(r"Phone: (?P<Phone>\+1-\d{3}-\d{3}-\d{4})\n").unwrap().captures(s);
    match (cv.is_none(),cp.is_none()) {
        (true,true) => "ERROR: VERSION or PHONE".to_string(),
        _ => {
            let mut v = version; let p = "+1-503-555-0090";
            let text1 = cv.unwrap().get(1).map(|m| m.as_str()).unwrap();
            if text1 == "2.0" {
                v = "2.0";
            }
            format!( "Program: {} Author: g964 Phone: {} Date: 2019-01-01 Version: {}", prog, p, v)
        },
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(s: &str, prog: &str, version: &str, exp: &str) -> () {
        println!("s:{:?}", s);
        println!("prog:{:?}", prog);
        println!("version:{:?}", version);
        let ans = change(s, prog, version);
        println!("actual: {:?}", ans);
        println!("expect: {:?}", exp);
        println!("{}", ans == exp);
        assert_eq!(ans, exp);
        println!("{}", "-");
    }

    #[test]
    fn basic_tests() {
        let s1="Program title: Primes\nAuthor: Kern\nCorporation: Gold\nPhone: +1-503-555-0091\nDate: Tues April 9, 2005\nVersion: 6.7\nLevel: Alpha";
        dotest(s1, "Ladder", "1.1", "Program: Ladder Author: g964 Phone: +1-503-555-0090 Date: 2019-01-01 Version: 1.1");
        let s2="Program title: Balance\nAuthor: Dorries\nCorporation: Funny\nPhone: +1-503-555-0095\nDate: Tues July 19, 2014\nVersion: 6.7\nLevel: Release";
        dotest(s2, "Circular", "1.5", "Program: Circular Author: g964 Phone: +1-503-555-0090 Date: 2019-01-01 Version: 1.5");
        let s13="Program title: Primes\nAuthor: Kern\nCorporation: Gold\nPhone: +1-503-555-0090\nDate: Tues April 9, 2005\nVersion: 67\nLevel: Alpha";
        dotest(s13, "Ladder", "1.1", "ERROR: VERSION or PHONE");

    }
}

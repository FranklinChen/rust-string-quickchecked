#![feature(phase)]

extern crate quickcheck;
#[phase(plugin)]
extern crate quickcheck_macros;

#[cfg(not(test))]
fn main() {
    println!("\"bananas\".contains(\"ana\") = {}",
             "bananas".contains("ana"))
}

#[cfg(test)]
mod test {
    use quickcheck::TestResult;

    #[test]
    fn example_good_substring() {
        assert!("bananas".contains("ana"))
    }

    #[quickcheck]
    fn all_good_substrings(left: String,
                           middle: String,
                           right: String) -> TestResult {
        let all = left
            .append(middle.as_slice())
            .append(right.as_slice());
        TestResult::from_bool(all.as_slice().contains(middle.as_slice()))
    }
}

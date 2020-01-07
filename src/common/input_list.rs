use std::fs;
use std::io::Result as IoResult;
use std::path::Path;
use std::str::FromStr;

/// Contains an Advent of Code input list (usually a string where each input is on its own line)
pub struct InputList {
    raw: String,
}

impl InputList {
    /// Construct a new input list from string slice
    pub fn new(raw: &str) -> InputList {
        InputList::new_from_str(String::from(raw))
    }

    /// Construct a new input list from a String
    pub fn new_from_str(raw: String) -> InputList {
        InputList { raw }
    }

    /// Create a new input list from a file
    pub fn new_from_file<P>(path: P) -> IoResult<InputList>
    where
        P: AsRef<Path>,
    {
        Ok(InputList::new_from_str(fs::read_to_string(path)?))
    }

    /// Parse the raw input list into a vector of some other type (delegates to `std::str::parse`)
    ///
    /// # Examples
    ///
    /// Given an input list of numbers, you can parse it into a Vec<i32>:
    ///
    /// ```
    /// use advent_of_code_2019::common::input_list;
    ///
    /// let num_input_list = input_list::InputList::new("123\n234\n345");
    /// let num_result: Result<Vec<i32>, _> = num_input_list.parse();
    /// let nums = num_result.unwrap();
    /// assert_eq!(nums.len(), 3);
    /// assert_eq!(nums[0], 123);
    /// assert_eq!(nums[1], 234);
    /// assert_eq!(nums[2], 345);
    /// ```
    pub fn parse<F: FromStr>(&self) -> Result<Vec<F>, F::Err> {
        self.raw
            .lines()
            .into_iter()
            .map(|s| str::parse::<F>(s))
            .collect()
    }

    pub fn raw(&self) -> &String {
        &self.raw
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_new_input_list() {
        InputList::new("");
        InputList::new("test");
        InputList::new("test\n1\ndog");
        InputList::new("123\n234\n345");
    }

    #[test]
    fn can_parse_input_list() {
        // Construct some new input lists; one is a valid list of numbers and the other is not.
        let num_input_list = InputList::new("123\n234\n345");
        let bad_input_list = InputList::new("cat\ndog\nnotanumber");
        // Attempt to parse both into a Vector of integers.
        let num_result: Result<Vec<i32>, _> = num_input_list.parse();
        let bad_result: Result<Vec<i32>, _> = bad_input_list.parse();
        // Ensure the valid one was successful and the invalid one wasn't.
        assert!(num_result.is_ok());
        assert!(bad_result.is_err());
        // Verify the valid one was correctly parsed
        let nums = num_result.unwrap();
        assert_eq!(nums.len(), 3);
        assert_eq!(nums[0], 123);
        assert_eq!(nums[1], 234);
        assert_eq!(nums[2], 345);
    }

    #[test]
    fn can_get_lines_iter() {
        let words_input_list = InputList::new("cat\ndog\nnumber");
        let mut words_lines = words_input_list.raw().lines();
        assert_eq!(words_lines.next(), Some("cat"));
        assert_eq!(words_lines.next(), Some("dog"));
        assert_eq!(words_lines.next(), Some("number"));
        assert_eq!(words_lines.next(), None);
    }
}

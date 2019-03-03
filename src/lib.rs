use chrono::NaiveDate;
use std::io::{Error, ErrorKind};
use std::str;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Pesel(String);

impl Pesel {
    /// Returns boolean representing if PESEL number is valid.
    pub fn validate(input: &str) -> bool {
        let result = if Pesel::validate_length(&input) && Pesel::validate_numeric(&input) {
            Pesel::validate_date(&input) && Pesel::validate_checksum(&input)
        } else {
            false
        };
        result
    }

    fn validate_length(input: &str) -> bool {
        let length = input.len();
        if length != 11 {
            println!("Invalid length.");
            false
        } else {
            true
        }
    }

    fn validate_numeric(input: &str) -> bool {
        input.chars().all(|c| c.is_digit(10))
    }

    fn validate_checksum(input: &str) -> bool {
        let numbers: Vec<u32> = input
            .chars()
            .map(|c| {
                c.to_digit(10)
                    .expect("validate_numeric function has to prevent panic here")
            })
            .collect();
        let weights = [1, 3, 7, 9, 1, 3, 7, 9, 1, 3];

        let pairs: Vec<_> = weights.iter().zip(numbers.iter()).collect();
        let products: Vec<_> = pairs.iter().map(|x| x.0 * x.1).collect();
        let partial_sum: u32 = products.iter().sum();
        let checksum: u32 = partial_sum + numbers.last().unwrap();

        checksum % 10 == 0
    }

    fn validate_date(input: &str) -> bool {
        let encoded_year = &input[0..2];
        let encoded_month = &input[2..4];
        let encoded_day = &input[4..6];

        let century: i32;
        match Pesel::get_century(&encoded_month) {
            Some(e) => century = e,
            None => return false,
        };

        let year = Pesel::get_year(century, &encoded_year);
        let month = Pesel::get_month(&encoded_month);
        let day: u32 = encoded_day
            .parse()
            .expect("validate_numeric function has to prevent panic here");

        let date = NaiveDate::from_ymd_opt(year, month, day);

        date.is_some()
    }

    fn get_century(encoded_month: &str) -> Option<i32> {
        let century: Option<i32> = {
            let century_number: u32 = encoded_month
                .parse()
                .expect("validate_numeric function has to prevent panic here");
            match century_number / 20 {
                0 => Some(1900),
                1 => Some(2000),
                2 => Some(2100),
                3 => Some(2200),
                4 => Some(1800),
                _ => None,
            }
        };

        century
    }

    fn get_year(epoch: i32, encoded_year: &str) -> i32 {
        let year_part: i32 = encoded_year
            .parse()
            .expect("validate_numeric function has to prevent panic here");

        epoch + year_part
    }

    fn get_month(encoded_month: &str) -> u32 {
        let month: u32 = encoded_month
            .parse()
            .expect("validate_numeric function has to prevent panic here");

        month % 20
    }
}

impl str::FromStr for Pesel {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let result = Pesel::validate(input);

        if result {
            Ok(Pesel(input.to_owned()))
        } else {
            Err(Error::new(ErrorKind::InvalidData, "Invalid Pesel"))
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn validates_true_pesel() {
        let result = crate::Pesel::validate("02070803628"); // this pesel was taken from official example page of polish gov
        assert_eq!(result, true);
    }

    #[test]
    fn rejects_if_too_short() {
        let result = crate::Pesel::validate("123");
        assert_eq!(result, false);
    }

    #[test]
    fn rejects_if_letters_present() {
        let result = crate::Pesel::validate("notapesel");
        assert_eq!(result, false);
    }

    #[test]
    fn rejects_if_wrong_checksum() {
        let result = crate::Pesel::validate("02070803629");
        assert_eq!(result, false);
    }

    #[test]
    fn rejects_if_wrong_encoded_date() {
        let result = crate::Pesel::validate("02130803629"); // TODO: find better example (with valid checksum)
        assert_eq!(result, false);
    }
}

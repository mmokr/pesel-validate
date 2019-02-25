pub struct Pesel(String);

impl Pesel {

    /// Returns boolean representing if PESEL number is valid.
    pub fn validate(input: &str) -> bool {
        true
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


use regex::Regex;

pub trait ConvertCase {
    fn to_pascal_case(&self) -> Self;
}

impl ConvertCase for String {
    fn to_pascal_case(&self) -> Self {
        let result = &self.to_lowercase();

        // 1. removes all - and _
        let re = Regex::new(r"[-_]+").unwrap();
        let result = re.replace_all(result.to_owned().as_str(), " ").to_string();

        let re = Regex::new(r"[^\w\s]").unwrap();
        let result = re.replace_all(result.to_owned().as_str(), "").to_string();

        let re = Regex::new(r"\s*(.)(\w*)").unwrap();
        let captures = re.captures_iter(result.as_str());

        captures.into_iter().fold("".to_string(), |cur, next| {
            let word = format!(
                "{}{}",
                next.get(1).unwrap().as_str().to_uppercase(),
                next.get(2).unwrap().as_str()
            );
            cur + word.as_str()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn words_test() {
        let test_string = "a b_c D/e+2-123";
        words(test_string.to_string())
    }

    #[test]
    fn pascal_case() {
        let test_string = "hello world".to_string();
        assert_eq!(test_string.to_pascal_case(), "HelloWorld");

        let test_string = "hello world-a-b".to_string();
        assert_eq!(test_string.to_pascal_case(), "HelloWorldAB");

        let test_string = "Hello World".to_string();
        assert_eq!(test_string.to_pascal_case(), "HelloWorld");
    }
}

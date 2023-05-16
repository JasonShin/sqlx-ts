pub mod sandbox;

pub mod test_utils {
    use regex::Regex;

    pub trait TSString {
        fn flatten(&self) -> Self;
    }

    impl TSString for String {
        fn flatten(&self) -> Self {
            let re = Regex::new(r"[\n\s]+").unwrap();
            re.replace_all(&self, " ").trim().to_string()
        }
    }

    #[test]
    fn test1() {
        let some_js = r#"
export type SomeQueryParams = [number, number, number];

export interface ISomeQueryResult {
    id: number;
    table_id: number;
    food_type: string;
    points: number;
    time_takes_to_cook: number;
};
        "#;

        let expected = "export type SomeQueryParams = [number, number, number]; export interface ISomeQueryResult { id: number; table_id: number; food_type: string; points: number; time_takes_to_cook: number; };";
        let result = some_js.to_string().flatten();

        assert_eq!(expected, result);
    }
}

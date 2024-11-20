#[cfg(test)]
mod tests {
    use pest::Parser;
    use rieltor_parser::{load_html, ApartmentParser, Rule};

    #[test]
    fn test_list_parsing() {
        let content = load_html(r"tests\ex2.html").unwrap();
        let pairs = ApartmentParser::parse(Rule::apartment_list, content.as_str())
            .expect("Unsuccessful parsing.");

        assert_eq!(pairs.clone().count(), 76);

        for pair in pairs {
            assert_eq!(pair.as_rule(), Rule::apartment_link);
        }
    }
}

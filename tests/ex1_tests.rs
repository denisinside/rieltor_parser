#[cfg(test)]
mod tests {
    use pest::Parser;
    use rieltor_parser::{load_html, ApartmentParser, Rule};



    #[test]
    fn test_document_parsing() {
        let content = load_html("ex1.html");
        let pairs = ApartmentParser::parse(Rule::document, &content).expect("Unsuccessful parsing.");

        for pair in pairs {
            match pair.as_rule() {
                Rule::price => {
                    let price = pair.as_str();
                    assert!(price.contains("9 000 грн/міс"), "The price does not match");
                }
                Rule::address => {
                    let address = pair.into_inner().next().unwrap().as_str();
                    assert!(address.eq("Петропавловсклівська"), "The address does not match");
                }
                Rule::det_descr_text => {
                    for inner_pair in pair.into_inner() {
                        match inner_pair.as_rule() {
                            Rule::planning_value => {
                                let room_planning = inner_pair.as_str();
                                assert_eq!(room_planning, "Роздільне", "The room planning does not match");
                            }
                            Rule::state_value => {
                                let state = inner_pair.as_str();
                                assert_eq!(state, "Хороший стан", "The apartment state does not match");
                            }
                            _ => {}
                        }
                    }
                }
                Rule::rieltor => {
                    for inner_pair in pair.into_inner() {
                        if inner_pair.as_rule() == Rule::rieltor_name {
                            let rieltor_name = inner_pair.as_str();
                            assert!(rieltor_name.contains("Силова Ірина"), "The rieltor name does not match");
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
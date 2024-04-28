
fn main() {

    println!("\n=============================\n");
    println!("Note: please run `cargo test` to test the library functionality, Thanks.");
    println!("\n=============================\n");
    
}


#[cfg(test)]
mod tests {
    use std::fs::OpenOptions;
    use std::io::Read;
    use async_graphql_parser::types::OperationDefinition;

    fn get_query_operation(name: &str) ->  OperationDefinition{
        let mut data_file = OpenOptions::new()
        .read(true)
        .open(name)
        .expect("cannot open file");

    let mut data = String::new();
    data_file.read_to_string(&mut data).expect("read failed");

    let operation = async_graphql_parser::parse_query(data)
        .unwrap()
        .operations
        .iter()
        .next()
        .unwrap()
        .1
        .node
        .clone();

        return operation
    }

    #[test]
    fn test_resolver_function() {
        let name = "details.more.gender";
        let operation = get_query_operation("query.graphql");
        let key_values: Vec<(String, String)> = vec![
            (String::from("gender"), String::from("male")),
            (String::from("city"), String::from("lahore")),
            (String::from("age"), String::from("20")),
        ];

        match resolver::resolve_op_value(name, &operation, &key_values) {
            Ok((placeholder_variable, substitution)) => {
                println!("placeholder_variable {}, substitution {}", placeholder_variable, substitution);
                assert!(placeholder_variable.eq(&"gender".to_string()));
                assert!(substitution.eq(&"male".to_string()));
            }
            Err(error_code) => {
                println!("ERROR! {}", resolver::error_code_message(error_code));
            }
        }
    }

    #[test]
    fn test_resolver_error_0() {
        let name = "";
        let operation = get_query_operation("query.graphql");
        let key_values: Vec<(String, String)> = vec![
            (String::from("gender"), String::from("male")),
            (String::from("city"), String::from("lahore")),
            (String::from("age"), String::from("20")),
        ];

        match resolver::resolve_op_value(name, &operation, &key_values) {
            Ok((placeholder_variable, substitution)) => {
                println!("placeholder_variable {}, substitution {}", placeholder_variable, substitution);
            }
            Err(error_code) => {
                println!("ERROR! {}", resolver::error_code_message(error_code));
                assert!(error_code==0);
            }
        }
    }

    #[test]
    fn test_resolver_error_1() {
        let name = "details.more.tail";
        let operation = get_query_operation("query.graphql");
        let key_values: Vec<(String, String)> = vec![
            (String::from("gender"), String::from("male")),
            (String::from("city"), String::from("lahore")),
            (String::from("age"), String::from("20")),
        ];

        match resolver::resolve_op_value(name, &operation, &key_values) {
            Ok((placeholder_variable, substitution)) => {
                println!("placeholder_variable {}, substitution {}", placeholder_variable, substitution);
            }
            Err(error_code) => {
                println!("ERROR! {}", resolver::error_code_message(error_code));
                assert!(error_code==1);
            }
        }
    }

    #[test]
    fn test_resolver_error_2() {
        let name = "details.more.tail";
        let operation = get_query_operation("query.graphql");
        let key_values: Vec<(String, String)> = vec![
            (String::from("gender"), String::from("male")),
            (String::from("city"), String::from("lahore")),
            (String::from("age"), String::from("20")),
            (String::from("tail"), String::from("1")),
        ];

        match resolver::resolve_op_value(name, &operation, &key_values) {
            Ok((placeholder_variable, substitution)) => {
                println!("placeholder_variable {}, substitution {}", placeholder_variable, substitution);
            }
            Err(error_code) => {
                println!("ERROR! {}", resolver::error_code_message(error_code));
                assert!(error_code==2);
            }
        }
    }
}
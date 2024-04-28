use async_graphql_parser::types::{Field, OperationDefinition};
use std::fs::OpenOptions;
use std::io::Read;

fn main() {
    let name = "details.more.gender";

    let mut data_file = OpenOptions::new()
        .read(true)
        .open("query.graphql")
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

    let key_values: Vec<(String, String)> = vec![
        (String::from("gender"), String::from("male")),
        (String::from("city"), String::from("lahore")),
        (String::from("age"), String::from("20")),
        (String::from("weight"), String::from("6")),
    ];

    match resolver::resolve_op_value(name, &operation, &key_values) {
        Ok((placeholder_value_before, placeholder_value_after)) => {}
        Err(error_code) => {
            println!("ERROR! {}", resolver::error_code_message(error_code));
        }
    }
}

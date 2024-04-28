use async_graphql_parser::types::OperationDefinition;
use async_graphql_parser::types::Selection;
use async_graphql_parser::Positioned;
use async_graphql_value::Name;
use async_graphql_value::Value;
use error_codes::*;
mod error_codes;

pub fn resolve_op_value(
    name: &str,
    operation: &OperationDefinition,
    key_values: &Vec<(String, String)>,
) -> Result<(String, String), u8> {
    // println!("name {}", name);
    // println!("key_values {:?}", key_values);
    // println!("operation {:?}", operation);

    if name.is_empty() {
        return Err(TARGET_NOT_FOUND);
    }

    let levels: Vec<String> = name.split(".").map(|s| s.to_string()).collect();
    println!("levels: {:?}", levels);

    let resolution_target = levels.last().unwrap();
    let mut substitution = None;
    for (key, value) in key_values.iter() {
        if key == resolution_target {
            substitution = Some(value);
            break;
        }
    }
    if substitution.is_none() {
        return Err(SUBSTITUTION_NOT_FOUND);
    }

    let selection_set = operation.selection_set.clone().into_inner().items;
    for selection in selection_set {
        let selection = selection.node;
        match selection {
            Selection::Field(field) => {
                let field_args = field.node.clone().arguments;

                let placeholder_value = get_placeholder_value(&field_args, levels.clone());
            }
            _ => {}
        }
    }

    Ok((String::from("xyz"), substitution.unwrap().to_string()))
}

fn get_placeholder_value(
    field_args: &Vec<(Positioned<Name>, Positioned<Value>)>,
    levels: Vec<String>,
) -> Result<String, u8> {
    for x in field_args.iter() {
        let value = x.1.node.clone();
        match value {
            Value::Boolean(x) => {}
            _ => {}
        };
        println!("x {:?} {:?}", x.0.node.as_str(), x.1.node);
    }
    Ok(String::from("$abc"))
}

pub fn error_code_message(error_code: u8) -> String {
    format!("error_code {}", error_code)
}

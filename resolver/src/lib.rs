use async_graphql_parser::types::{OperationDefinition, Selection};
use async_graphql_value::{Name, Value, indexmap};
use error_codes::*;
use indexmap::IndexMap;

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
    // println!("levels: {:?}", levels);

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
    let mut placeholder_value = String::default();
    for selection in selection_set {
        let selection = selection.node;
        match selection {
            Selection::Field(field) => {
                let field_args = field.node.clone().arguments;

                let args: Vec<(Name, Value)> = field_args
                .iter()
                .map(|(a,b)| {
                    let x = a.node.clone();
                    let y = b.node.clone();
                    (x,y)
                })
                .clone()
                .collect();

                let mut fields_map: IndexMap<Name, Value> = IndexMap::new();
                for (name, value) in args.iter() {
                    fields_map.insert(name.clone(), value.clone());
                }

                let placeholder_prev_value = get_placeholder_value(&fields_map, &mut levels.clone());
                if let Ok(value) = placeholder_prev_value {
                    placeholder_value = value;
                } else {
                    return Err(NAME_NOT_FOUND);
                }
            }
            _ => {}
        }
    }

    Ok((placeholder_value, substitution.unwrap().to_string()))
}

fn get_placeholder_value(
    field_map: &IndexMap<Name, Value>,
    levels: &mut Vec<String>,
) -> Result<String, u8> {
    let next_level = levels.iter().next().unwrap().clone();
    levels.remove(0);

    for (name, value) in field_map.iter() {
        if name.to_string() == next_level {
            // println!("levels.len() {}", levels.len());
            if levels.len() == 0 {
                match value.clone() {
                    Value::Variable(name) => {
                        // println!("name: {:?}", name.as_str());
                        return Ok(String::from(name.as_str()));
                    },
                    _ => {}
                }    
            } else {
                // println!("name {}", name.as_str());
                match value.clone() {
                    Value::Object(obj) => {
                        // println!("obj: {:?}", obj);
                        let fields_map = obj;
                        if let Ok(placeholder) = get_placeholder_value(&fields_map, levels) {
                            return Ok(placeholder);
                        } else {
                            return Err(NAME_NOT_FOUND);
                        }
                        // return Ok(get_placeholder_value(&fields_map, levels).unwrap());
                    },
                    _ => {}
                }
            }
        } 
    }
    return Err(NAME_NOT_FOUND);
}

pub fn error_code_message(error_code: u8) -> String {
    format!("error_code: {}", error_code)
}

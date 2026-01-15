use crate::errors::AppError;
use serde_json::{Value, json};

pub fn print_ok(result: Value) {
    print_yaml(&result);
}

pub fn print_error(error: &AppError) {
    let mut payload = json!({
        "code": error.code(),
        "message": error.message(),
    });

    if let Some(details) = error.details() {
        payload["details"] = details.clone();
    }

    print_yaml(&payload);
}

fn print_yaml(value: &Value) {
    match serde_saphyr::to_string(value) {
        Ok(yaml) => print!("{yaml}"),
        Err(err) => println!("ok: false\nerror:\n  code: yaml_encode\n  message: {err}"),
    }
}

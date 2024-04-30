use std::fs;

pub fn get_dataset() -> String {
    let data = fs::read_to_string("./dataset.json").expect("Unable to read file");
    let json: serde_json::Value = serde_json::from_str(&data).expect("JSON was not well-formatted");
    let mut text = String::new();

    if let Some(rows) = json.get("rows").and_then(serde_json::Value::as_array) {
        for row in rows {
            if let Some(row_obj) = row.get("row").and_then(serde_json::Value::as_object) {
                if let Some(text_val) = row_obj.get("text").and_then(serde_json::Value::as_str) {
                    text.push_str(text_val);
                }
            }
        }
    }
    text
}

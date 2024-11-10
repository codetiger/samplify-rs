use samplify_rs::Sampleable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Sampleable)]
pub struct SimpleReport {
    id: String,
    name: String,
    score: String,
    is_active: String,
}

fn main() -> Result<(), String> {
    let config_json = r#"
    {
        "id": {
            "type": "uuid"
        },
        "name": {
            "type": "name",
            "subtype": "name_with_title"
        },
        "score": {
            "type": "number",
            "params": {
                "min": 0,
                "max": 100,
                "float": true,
                "decimals": 2
            }
        },
        "is_active": {
            "type": "boolean",
            "params": {
                "true_probability": 0.7
            }
        }
    }"#;

    let config: serde_json::Map<String, serde_json::Value> = 
        serde_json::from_str(config_json).map_err(|e| e.to_string())?;

    let report = SimpleReport::sample_with_config(&config)?;
    println!("Generated Report:\n{:#?}", report);

    Ok(())
}
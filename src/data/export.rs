use serde_json::Value;
use std::collections::HashMap;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use crate::Result;

pub struct DataExporter {
    output_dir: String,
}

impl DataExporter {
    pub fn new(output_dir: String) -> Self {
        Self { output_dir }
    }

    pub async fn export_to_csv(&self, data: &[(String, Value)], filename: &str) -> Result<String> {
        let mut csv_content = String::new();
        
        if data.is_empty() {
            return Ok(String::new());
        }

        // Extract headers from first record
        let headers = self.extract_headers(&data[0].1);
        csv_content.push_str(&headers.join(","));
        csv_content.push('\n');

        // Export data rows
        for (url, record) in data {
            let mut row = Vec::new();
            row.push(format!("\"{}\"", url.replace("\"", "\"\"")));
            
            for header in &headers[1..] { // Skip URL header
                let value = self.extract_value(&record, header);
                row.push(format!("\"{}\"", value.replace("\"", "\"\"")));
            }
            
            csv_content.push_str(&row.join(","));
            csv_content.push('\n');
        }

        let filepath = format!("{}/{}", self.output_dir, filename);
        self.ensure_output_dir().await?;
        
        let mut file = tokio::fs::File::create(&filepath).await?;
        file.write_all(csv_content.as_bytes()).await?;

        Ok(filepath)
    }

    pub async fn export_to_json(&self, data: &[(String, Value)], filename: &str) -> Result<String> {
        let mut json_data = Vec::new();
        
        for (url, record) in data {
            let mut entry = HashMap::new();
            entry.insert("url".to_string(), Value::String(url.clone()));
            
            if let Value::Object(obj) = record {
                for (key, value) in obj {
                    entry.insert(key.clone(), value.clone());
                }
            }
            
            json_data.push(Value::Object(entry.into_iter().map(|(k, v)| (k, v)).collect()));
        }

        let json_content = serde_json::to_string_pretty(&json_data)?;
        let filepath = format!("{}/{}", self.output_dir, filename);
        self.ensure_output_dir().await?;
        
        let mut file = tokio::fs::File::create(&filepath).await?;
        file.write_all(json_content.as_bytes()).await?;

        Ok(filepath)
    }

    pub async fn export_to_xml(&self, data: &[(String, Value)], filename: &str) -> Result<String> {
        let mut xml_content = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<records>\n");
        
        for (url, record) in data {
            xml_content.push_str("  <record>\n");
            xml_content.push_str(&format!("    <url>{}</url>\n", self.escape_xml(url)));
            
            if let Value::Object(obj) = record {
                for (key, value) in obj {
                    let value_str = match value {
                        Value::String(s) => s.clone(),
                        Value::Number(n) => n.to_string(),
                        Value::Bool(b) => b.to_string(),
                        _ => serde_json::to_string(value).unwrap_or_default(),
                    };
                    xml_content.push_str(&format!("    <{}>{}</{}>\n", 
                        key, self.escape_xml(&value_str), key));
                }
            }
            
            xml_content.push_str("  </record>\n");
        }
        
        xml_content.push_str("</records>\n");

        let filepath = format!("{}/{}", self.output_dir, filename);
        self.ensure_output_dir().await?;
        
        let mut file = tokio::fs::File::create(&filepath).await?;
        file.write_all(xml_content.as_bytes()).await?;

        Ok(filepath)
    }

    async fn ensure_output_dir(&self) -> Result<()> {
        fs::create_dir_all(&self.output_dir).await?;
        Ok(())
    }

    fn extract_headers(&self, record: &Value) -> Vec<String> {
        let mut headers = vec!["url".to_string()];
        
        if let Value::Object(obj) = record {
            for key in obj.keys() {
                headers.push(key.clone());
            }
        }
        
        headers
    }

    fn extract_value(&self, record: &Value, key: &str) -> String {
        if let Value::Object(obj) = record {
            if let Some(value) = obj.get(key) {
                match value {
                    Value::String(s) => s.clone(),
                    Value::Number(n) => n.to_string(),
                    Value::Bool(b) => b.to_string(),
                    Value::Array(arr) => {
                        arr.iter()
                            .map(|v| match v {
                                Value::String(s) => s.clone(),
                                _ => serde_json::to_string(v).unwrap_or_default(),
                            })
                            .collect::<Vec<_>>()
                            .join("; ")
                    },
                    _ => serde_json::to_string(value).unwrap_or_default(),
                }
            } else {
                String::new()
            }
        } else {
            String::new()
        }
    }

    fn escape_xml(&self, text: &str) -> String {
        text.replace("&", "&amp;")
            .replace("<", "&lt;")
            .replace(">", "&gt;")
            .replace("\"", "&quot;")
            .replace("'", "&apos;")
    }
}

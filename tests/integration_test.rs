#[cfg(test)]
mod tests {
    use openapi_rs::Spec;
    use std::fs;
    use std::path::Path;

    fn get_spec_from_file(path: &Path) -> Spec {
        let file = fs::read(path).unwrap();
        let file_content = String::from_utf8(file).unwrap();
        serde_json::from_str(&file_content).unwrap()
    }

    fn minify_json(raw_json: &str) -> Result<String, serde_json::Error> {
        let parsed: serde_json::Value = serde_json::from_str(raw_json)?;
        let minified = serde_json::to_string(&parsed)?;
        Ok(minified)
    }

    #[test]
    fn test_openai_serialization_json() {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let path = Path::new(manifest_dir).join("tests/fixtures/openapi.json");
        let spec = get_spec_from_file(&path);
        let serialized = serde_json::to_string(&spec).unwrap();
        let file = fs::read(path).unwrap();
        let file_content = String::from_utf8(file).unwrap();
        assert_eq!(
            minify_json(&serialized).unwrap(),
            minify_json(&file_content).unwrap()
        );
    }
}

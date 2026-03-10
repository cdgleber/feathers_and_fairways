pub fn generate_access_key() -> String {
    petname::petname(3, "-").unwrap_or("unexpected-no-names".to_string())
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ContactObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

pub type Contacts = std::collections::HashMap<String, ContactObject>;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_contact_serialize_deserialize_json() {
        let contact = ContactObject {
            name: Some("John Doe".to_string()),
            url: Some("http://example.com".to_string()),
            email: Some("john.doe@example.com".to_string()),
        };

        let serialized = serde_json::to_string(&contact).unwrap();
        let deserialized: ContactObject = serde_json::from_str(&serialized).unwrap();
        assert_eq!(contact, deserialized);
    }
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct OAuthFlowObject {
    #[serde(rename = "implicit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub implicit: Option<Box<OAuthFlowObject>>,

    #[serde(rename = "password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<Box<OAuthFlowObject>>,

    #[serde(rename = "clientCredentials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_credentials: Option<Box<OAuthFlowObject>>,

    #[serde(rename = "authorizationCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_code: Option<Box<OAuthFlowObject>>,

    #[serde(rename = "deviceAuthorization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_authorization: Option<Box<OAuthFlowObject>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_oauthflow_serialize_deserialize_json() {
        let oauth_flow = OAuthFlowObject::default();

        let json = serde_json::to_string(&oauth_flow).unwrap();
        let deserialized: OAuthFlowObject = serde_json::from_str(&json).unwrap();
        assert_eq!(oauth_flow, deserialized);
    }
}

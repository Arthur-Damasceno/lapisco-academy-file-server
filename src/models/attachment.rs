#[derive(Debug, Clone, Serialize)]
pub struct Attachment {
    #[serde(rename = "i")]
    pub id: String,
    #[serde(rename = "n")]
    pub filename: String,
    #[serde(skip_serializing)]
    pub content_type: String,
}

impl Attachment {
    pub fn new(filename: String, content_type: String) -> Self {
        Self {
            id: nanoid!(12),
            filename,
            content_type,
        }
    }
}

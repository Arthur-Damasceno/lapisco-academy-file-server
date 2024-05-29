#[derive(Debug, Clone, Serialize)]
pub struct Attachment {
    #[serde(rename = "i")]
    pub id: String,
    #[serde(rename = "e")]
    pub extension: AttachmentExtension,
}

impl Attachment {
    pub fn new(extension: AttachmentExtension) -> Self {
        Self {
            id: nanoid!(12),
            extension,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize_repr)]
#[repr(u8)]
pub enum AttachmentExtension {
    Mp4 = 0,
}

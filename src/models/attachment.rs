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

    pub fn filename(&self) -> String {
        let Self { id, extension } = self;

        match extension {
            AttachmentExtension::Mp4 => format!("{id}.mp4"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize_repr)]
#[repr(u8)]
pub enum AttachmentExtension {
    Mp4 = 0,
}

impl AttachmentExtension {
    pub fn content_type(&self) -> &str {
        match self {
            Self::Mp4 => "video/mp4",
        }
    }
}

impl From<i64> for AttachmentExtension {
    fn from(value: i64) -> Self {
        match value {
            0 => Self::Mp4,
            _ => unreachable!(),
        }
    }
}

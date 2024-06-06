use std::fmt;

use crate::error::{Error, Result};

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

        format!("{id}.{extension}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize_repr)]
#[repr(u8)]
pub enum AttachmentExtension {
    Mp4 = 0,
    Pdf = 1,
}

impl AttachmentExtension {
    pub fn content_type(&self) -> &'static str {
        match self {
            Self::Mp4 => "video/mp4",
            Self::Pdf => "application/pdf",
        }
    }
}

impl TryFrom<&str> for AttachmentExtension {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        match value {
            "video/mp4" => Ok(Self::Mp4),
            "application/pdf" => Ok(Self::Pdf),
            _ => Err(Error::InvalidData),
        }
    }
}

impl fmt::Display for AttachmentExtension {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Mp4 => "mp4",
                Self::Pdf => "pdf",
            }
        )
    }
}

impl From<i64> for AttachmentExtension {
    fn from(value: i64) -> Self {
        match value {
            0 => Self::Mp4,
            1 => Self::Pdf,
            _ => unreachable!(),
        }
    }
}

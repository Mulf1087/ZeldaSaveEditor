use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum SaveError {
    UnknownFormat,
    MissingField(&'static str),
    IndexOutOfRange { index: usize, max: usize },
    Truncated { offset: usize, len: usize },
    /// A write to a fixed-size slot was given data of the wrong length. Currently only
    /// AutoBuild's `CombinedActorInfo` binary blob, whose stored per-slot length is never
    /// resized (matching the source's `writeBinary`), so writing the wrong size would either
    /// leave stale trailing bytes or overwrite the start of the next slot's length header.
    SizeMismatch { expected: usize, actual: usize },
}

impl fmt::Display for SaveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SaveError::UnknownFormat => write!(f, "unrecognized save file format"),
            SaveError::MissingField(name) => {
                write!(f, "expected field not found in hash table: {name}")
            }
            SaveError::IndexOutOfRange { index, max } => {
                write!(f, "index {index} out of range (max {max})")
            }
            SaveError::Truncated { offset, len } => {
                write!(f, "attempted to read/write past end of buffer (offset {offset}, buffer is {len} bytes)")
            }
            SaveError::SizeMismatch { expected, actual } => {
                write!(f, "expected {expected} bytes, got {actual}")
            }
        }
    }
}

impl std::error::Error for SaveError {}

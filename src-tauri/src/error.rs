use serde::Serialize;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct ShellError {
    pub kind: String,
    pub message: String,
}

impl From<save_engine::SaveError> for ShellError {
    fn from(err: save_engine::SaveError) -> Self {
        let kind = match &err {
            save_engine::SaveError::UnknownFormat => "unknown_format",
            save_engine::SaveError::MissingField(_) => "missing_field",
            save_engine::SaveError::IndexOutOfRange { .. } => "index_out_of_range",
            save_engine::SaveError::Truncated { .. } => "truncated",
            save_engine::SaveError::SizeMismatch { .. } => "size_mismatch",
        };
        ShellError { kind: kind.to_string(), message: err.to_string() }
    }
}

impl ShellError {
    pub fn no_save_loaded() -> Self {
        ShellError {
            kind: "no_save_loaded".to_string(),
            message: "no save file is currently open".to_string(),
        }
    }

    pub fn wrong_game(expected: &str) -> Self {
        ShellError {
            kind: "wrong_game".to_string(),
            message: format!("the currently open save is not a {expected} save"),
        }
    }

    pub fn dialog_cancelled() -> Self {
        ShellError {
            kind: "dialog_cancelled".to_string(),
            message: "no file was selected".to_string(),
        }
    }

    pub fn io(err: std::io::Error) -> Self {
        ShellError { kind: "io_error".to_string(), message: err.to_string() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use save_engine::SaveError;

    #[test]
    fn missing_field_maps_to_missing_field_kind() {
        let shell_err: ShellError = SaveError::MissingField("RUPEES").into();
        assert_eq!(shell_err.kind, "missing_field");
        assert!(shell_err.message.contains("RUPEES"));
    }

    #[test]
    fn index_out_of_range_maps_to_index_out_of_range_kind() {
        let shell_err: ShellError = SaveError::IndexOutOfRange { index: 5, max: 3 }.into();
        assert_eq!(shell_err.kind, "index_out_of_range");
    }
}

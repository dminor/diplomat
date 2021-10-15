use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CallableLibraryType {
    pub name: String,
    pub expr: String,
    pub is_call: bool,
}

#[derive(Debug, Deserialize)]
pub struct LibraryType {
    pub name: String,
    pub expr: String,
}

#[derive(Debug, Deserialize)]
pub struct LibraryConfig {
    pub headers: Vec<String>,
    pub nullopt: CallableLibraryType,
    pub optional: LibraryType,
    pub someopt: CallableLibraryType,
    pub span: LibraryType,
    pub string_view: LibraryType,
    pub unique_ptr: LibraryType,
}

impl LibraryConfig {
    pub fn default() -> LibraryConfig {
        LibraryConfig {
            headers: vec!["#include <optional>".into(), "#include <span>".into()],
            nullopt: CallableLibraryType {
                name: "std::nullopt".into(),
                expr: "std::nullopt".into(),
                is_call: false,
            },
            optional: LibraryType {
                name: "std::optional".into(),
                expr: "std::optional".into(),
            },
            someopt: CallableLibraryType {
                name: "".into(),
                expr: "".into(),
                is_call: false,
            },
            span: LibraryType {
                name: "std::span".into(),
                expr: "std::span".into(),
            },
            string_view: LibraryType {
                name: "std::string_view".into(),
                expr: "std::string_view".into(),
            },
            unique_ptr: LibraryType {
                name: "std::unique_ptr".into(),
                expr: "std::unique_ptr".into(),
            },
        }
    }
}
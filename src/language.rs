use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Language {
    Cpp, Rust, Python, JavaScript, Java, CSharp,
}

impl Language {
    pub fn from_alias(s: &str) -> Option<Self> {
        match s.trim().to_ascii_lowercase().as_str() {
            "cpp" | "c++" | "cxx" => Some(Self::Cpp),
            "rust" | "rs" => Some(Self::Rust),
            "python" | "py" => Some(Self::Python),
            "javascript" | "js" | "node" => Some(Self::JavaScript),
            "java" => Some(Self::Java),
            "csharp" | "c#" | "cs" => Some(Self::CSharp),
            _ => None,
        }
    }

    pub fn key(self) -> &'static str {
        match self {
            Self::Cpp => "cpp", Self::Rust => "rust", Self::Python => "python",
            Self::JavaScript => "javascript", Self::Java => "java",
            Self::CSharp => "csharp",
        }
    }

    pub fn display(self) -> &'static str {
        match self {
            Self::Cpp => "C++", Self::Rust => "Rust", Self::Python => "Python",
            Self::JavaScript => "JavaScript", Self::Java => "Java",
            Self::CSharp => "C#",
        }
    }

    pub fn comment_prefix(self) -> &'static str {
        match self {
            Self::Python => "#",
            _ => "//",
        }
    }

    pub fn extension(self) -> &'static str {
        match self {
            Self::Cpp => "cpp", Self::Rust => "rs", Self::Python => "py",
            Self::JavaScript => "js", Self::Java => "java",
            Self::CSharp => "cs",
        }
    }
}


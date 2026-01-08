//! Error types for YATR
//!
//! Uses `miette` for pretty error reporting with source spans and help text.

use miette::Diagnostic;
use std::path::PathBuf;
use thiserror::Error;

/// Main error type for YATR operations
#[derive(Error, Diagnostic, Debug)]
pub enum YatrError {
    #[error("Configuration file not found")]
    #[diagnostic(help("Create a yatr.toml in your project root, or specify one with --config"))]
    ConfigNotFound { searched: Vec<PathBuf> },

    #[error("Failed to parse configuration")]
    ConfigParse {
        #[source]
        source: toml::de::Error,
        path: PathBuf,
    },

    #[error("Task '{name}' not found")]
    #[diagnostic(help("Run `yatr list` to see available tasks"))]
    TaskNotFound {
        name: String,
        available: Vec<String>,
    },

    #[error("Circular dependency detected: {cycle}")]
    #[diagnostic(help("Check the 'depends' field in your task definitions"))]
    CyclicDependency { cycle: String },

    #[error("Task '{task}' failed with exit code {code}")]
    TaskFailed {
        task: String,
        code: i32,
        #[help]
        stderr: Option<String>,
    },

    #[error("Command not found: {command}")]
    #[diagnostic(help("Ensure the command is installed and in your PATH"))]
    CommandNotFound { command: String },

    #[error("Script execution failed in task '{task}'")]
    ScriptFailed {
        task: String,
        #[source]
        source: Box<rhai::EvalAltResult>,
    },

    #[error("Invalid task configuration")]
    InvalidTask { task: String, reason: String },

    #[error("I/O error")]
    Io(#[from] std::io::Error),

    #[error("Cache error: {message}")]
    Cache { message: String },

    #[error("Watch error")]
    Watch {
        #[source]
        source: notify::Error,
    },
}

/// Result type alias for YATR operations
pub type Result<T> = std::result::Result<T, YatrError>;

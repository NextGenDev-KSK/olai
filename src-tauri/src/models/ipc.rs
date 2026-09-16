//! Input/output DTOs for the IPC boundary. All are `ts-rs`-exported so the
//! TypeScript client uses the exact same shapes.

use super::analysis::Analysis;
use super::document::{Document, DocRole};
use super::session::SessionId;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// One page of already-extracted PDF text (from the Web Worker).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/lib/bindings/")]
#[serde(rename_all = "camelCase")]
pub struct PageInput {
    /// 1-based page number.
    pub number: u32,
    /// Extracted text for the page.
    pub text: String,
}

/// How a file's content is delivered to the backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/lib/bindings/")]
#[serde(rename_all = "camelCase")]
pub enum IngestKind {
    /// A PDF: raw bytes for validation plus worker-extracted pages.
    Pdf,
    /// An image: raw bytes sent to vision OCR.
    Image,
}

/// A single file to ingest. `dataBase64` is the raw file bytes (for magic-byte
/// validation and hashing); PDFs also include `pages` from the worker.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/lib/bindings/")]
#[serde(rename_all = "camelCase")]
pub struct IngestFile {
    /// Original file name (basename only).
    pub file_name: String,
    /// The user-assigned role.
    pub role: DocRole,
    /// How the content is delivered.
    pub kind: IngestKind,
    /// Base64-encoded raw file bytes.
    pub data_base64: String,
    /// Worker-extracted PDF pages (absent for images).
    pub pages: Option<Vec<PageInput>>,
}

/// The result of loading a demo scenario.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/lib/bindings/")]
#[serde(rename_all = "camelCase")]
pub struct DemoSession {
    /// The created session id.
    pub session_id: SessionId,
    /// The demo documents.
    pub docs: Vec<Document>,
    /// The computed analysis.
    pub analysis: Analysis,
}

/// Which bundled demo scenario to load.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/lib/bindings/")]
#[serde(rename_all = "camelCase")]
pub enum DemoScenarioKind {
    /// Eviction notice vs rental agreement.
    Eviction,
    /// Scam notice.
    Scam,
}

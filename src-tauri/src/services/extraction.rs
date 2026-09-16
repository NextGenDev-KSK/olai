//! File intake validation: magic-byte sniffing, size/page/count caps, and
//! content hashing. Documents are untrusted; nothing here trusts a file
//! extension without confirming the bytes.

use crate::error::{AppError, AppResult};
use sha2::{Digest, Sha256};

/// Maximum number of documents per session.
pub const MAX_FILES: usize = 3;
/// Maximum pages per document.
pub const MAX_PAGES_PER_DOC: u32 = 15;
/// Maximum bytes per file (20 MB).
pub const MAX_BYTES: usize = 20 * 1024 * 1024;

/// A validated file type, confirmed by magic bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    /// PDF document.
    Pdf,
    /// PNG image.
    Png,
    /// JPEG image.
    Jpg,
}

impl FileType {
    /// Whether this type is an image (routed to vision OCR).
    pub fn is_image(self) -> bool {
        matches!(self, FileType::Png | FileType::Jpg)
    }
}

/// Detect a file's type from its leading magic bytes.
pub fn detect_file_type(bytes: &[u8]) -> Option<FileType> {
    if bytes.starts_with(b"%PDF-") {
        Some(FileType::Pdf)
    } else if bytes.starts_with(&[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]) {
        Some(FileType::Png)
    } else if bytes.starts_with(&[0xFF, 0xD8, 0xFF]) {
        Some(FileType::Jpg)
    } else {
        None
    }
}

/// Validate one uploaded file: size cap and magic-byte type detection.
pub fn validate_upload(bytes: &[u8]) -> AppResult<FileType> {
    if bytes.is_empty() {
        return Err(AppError::Validation("empty file".into()));
    }
    if bytes.len() > MAX_BYTES {
        return Err(AppError::LimitExceeded(format!(
            "file exceeds {} MB limit",
            MAX_BYTES / (1024 * 1024)
        )));
    }
    detect_file_type(bytes).ok_or(AppError::UnsupportedFile)
}

/// Enforce the per-session file-count cap.
pub fn validate_file_count(count: usize) -> AppResult<()> {
    if count > MAX_FILES {
        return Err(AppError::LimitExceeded(format!(
            "at most {MAX_FILES} files"
        )));
    }
    Ok(())
}

/// Enforce the per-document page cap.
pub fn validate_page_count(pages: u32) -> AppResult<()> {
    if pages == 0 {
        return Err(AppError::Validation("document has no pages".into()));
    }
    if pages > MAX_PAGES_PER_DOC {
        return Err(AppError::LimitExceeded(format!(
            "at most {MAX_PAGES_PER_DOC} pages per document"
        )));
    }
    Ok(())
}

/// SHA-256 hex digest of file bytes, used as a per-document cache key.
pub fn hash_bytes(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let digest = hasher.finalize();
    let mut out = String::with_capacity(digest.len() * 2);
    for byte in digest {
        out.push_str(&format!("{byte:02x}"));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_pdf_png_jpg() {
        assert_eq!(detect_file_type(b"%PDF-1.7\n..."), Some(FileType::Pdf));
        assert_eq!(
            detect_file_type(&[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00]),
            Some(FileType::Png)
        );
        assert_eq!(
            detect_file_type(&[0xFF, 0xD8, 0xFF, 0xE0]),
            Some(FileType::Jpg)
        );
        assert_eq!(detect_file_type(b"<html>"), None);
    }

    #[test]
    fn rejects_mismatched_or_oversize_files() {
        assert!(matches!(
            validate_upload(b"not a real file"),
            Err(AppError::UnsupportedFile)
        ));
        assert!(matches!(validate_upload(&[]), Err(AppError::Validation(_))));
        let big = vec![b'%'; MAX_BYTES + 1];
        assert!(matches!(
            validate_upload(&big),
            Err(AppError::LimitExceeded(_))
        ));
    }

    #[test]
    fn enforces_count_and_page_caps() {
        assert!(validate_file_count(3).is_ok());
        assert!(matches!(
            validate_file_count(4),
            Err(AppError::LimitExceeded(_))
        ));
        assert!(validate_page_count(15).is_ok());
        assert!(matches!(
            validate_page_count(16),
            Err(AppError::LimitExceeded(_))
        ));
        assert!(matches!(
            validate_page_count(0),
            Err(AppError::Validation(_))
        ));
    }

    #[test]
    fn image_type_is_flagged() {
        assert!(FileType::Png.is_image());
        assert!(!FileType::Pdf.is_image());
    }

    #[test]
    fn hash_is_stable_and_hex() {
        let h = hash_bytes(b"hello");
        assert_eq!(h.len(), 64);
        assert_eq!(h, hash_bytes(b"hello"));
        assert_ne!(h, hash_bytes(b"world"));
    }
}

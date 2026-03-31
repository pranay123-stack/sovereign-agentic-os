//! Kernel-specific error types.

use sovereign-agentic-os_types::error::Sovereign Agentic OSError;
use thiserror::Error;

/// Kernel error type wrapping Sovereign Agentic OSError with kernel-specific context.
#[derive(Error, Debug)]
pub enum KernelError {
    /// A wrapped Sovereign Agentic OSError.
    #[error(transparent)]
    Sovereign Agentic OS(#[from] Sovereign Agentic OSError),

    /// The kernel failed to boot.
    #[error("Boot failed: {0}")]
    BootFailed(String),
}

/// Alias for kernel results.
pub type KernelResult<T> = Result<T, KernelError>;

pub mod constants;
pub mod dtln_engine;
pub mod dtln_processor;
pub mod dtln_utilities;
pub mod tflite;

// Node.js bindings (conditional compilation)
#[cfg(feature = "nodejs")]
pub mod nodejs_bindings;

// Python bindings (conditional compilation)
#[cfg(feature = "python")]
pub mod python_bindings;

// Re-export for Node.js when that feature is enabled
#[cfg(feature = "nodejs")]
pub use nodejs_bindings::*;

// Re-export for Python when that feature is enabled
#[cfg(feature = "python")]
pub use python_bindings::*;

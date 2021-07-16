//! Allocation errors handling module

use std::alloc;
use std::error;
use std::fmt;

/// Crate's error type describing issues with allocation
/// 
/// Please check possible reasons of issues in [AllocErrorType]
/// 
/// Usage example:
/// ```rust
/// use fallible_alloc::vec::alloc_with_size;
///
/// match alloc_with_size::<f32>(10) {
///   Ok(vec) => println!("Created a vec with size 10"),
///   Err(error) => println!("Failed to create a vec, reason: {}", error)
/// }
/// ```
#[derive(fmt::Debug, Clone, Hash)]
pub struct AllocError {
    error_type: AllocErrorType,
    message: String,
}

/// Reasons of allocation issues
#[derive(fmt::Debug, Clone, PartialEq, Hash)]
pub enum AllocErrorType {
    /// Layout provided to std allocation functions 
    /// is incorrect
    LayoutError,
    /// Failed to allocate a memory with global allocator (it has returned null ptr)
    FailedAllocation,
}

impl fmt::Display for AllocErrorType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AllocErrorType::LayoutError => write!(formatter, "layouts error"),
            AllocErrorType::FailedAllocation => write!(formatter, "failed allocation"),
        }
    }
}

impl fmt::Display for AllocError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "error caused by {}, reason: {}",
            self.error_type, self.message
        )
    }
}

impl From<alloc::LayoutError> for AllocError {
    fn from(error: alloc::LayoutError) -> AllocError {
        AllocError::new(format!("{}", error), AllocErrorType::LayoutError)
    }
}

impl error::Error for AllocError {}

unsafe impl Send for AllocError {}

unsafe impl Sync for AllocError {}

impl AllocError {
    /// Creates a new [AllocError]
    /// 
    /// Please check error types in [AllocErrorType]
    /// 
    /// Usage example:
    /// ```rust
    /// use fallible_alloc::alloc_error::AllocError;
    /// use fallible_alloc::alloc_error::AllocErrorType;
    /// let error = AllocError::new("error", AllocErrorType::FailedAllocation);
    /// ```
    pub fn new<M: AsRef<str>>(message: M, error_type: AllocErrorType) -> AllocError {
        AllocError {
            error_type,
            message: message.as_ref().to_string(),
        }
    }

    /// [AllocError]'s message getter
    /// 
    /// Message is being cloned on return
    /// 
    /// Usage example:
    /// ```rust
    /// use fallible_alloc::alloc_error::AllocError;
    /// use fallible_alloc::alloc_error::AllocErrorType;
    /// let error = AllocError::new("error", AllocErrorType::FailedAllocation);
    /// println!("{}", error.message());
    /// ```
    pub fn message(&self) -> String {
        self.message.clone()
    }

    /// [AllocError]'s type getter
    /// 
    /// Message is being cloned on return
    /// 
    /// Usage example:
    /// ```rust
    /// use fallible_alloc::alloc_error::AllocError;
    /// use fallible_alloc::alloc_error::AllocErrorType;
    /// let error = AllocError::new("error", AllocErrorType::FailedAllocation);
    /// assert_eq!(error.error_type(), AllocErrorType::FailedAllocation);
    /// ```
    pub fn error_type(&self) -> AllocErrorType {
        self.error_type.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::AllocError;
    use super::AllocErrorType;
    use std::alloc::Layout;

    #[test]
    fn it_inits() {
        let layout_error = AllocError::new("layout error", AllocErrorType::LayoutError);

        assert_eq!(layout_error.message(), "layout error");
        assert_eq!(layout_error.error_type(), AllocErrorType::LayoutError);

        let alloc_error = AllocError::new("alloc error", AllocErrorType::FailedAllocation);

        assert_eq!(alloc_error.message(), "alloc error");
        assert_eq!(alloc_error.error_type(), AllocErrorType::FailedAllocation);
    }

    #[test]
    fn it_initable_from_layout_error() {
        // Using incorrect align intentionaly to produce an error
        match Layout::from_size_align(123, 3) {
            Ok(_) => panic!("Unable to produce a layout error"),
            Err(error) => {
                let error = AllocError::from(error);
                assert_eq!(error.error_type(), AllocErrorType::LayoutError);
            }
        }
    }

    #[test]
    fn it_displayable() {
        let layout_error = AllocError::new("layout error", AllocErrorType::LayoutError);

        assert_eq!(
            format!("{}", layout_error),
            "error caused by layouts error, reason: layout error"
        );

        let alloc_error = AllocError::new("alloc error", AllocErrorType::FailedAllocation);

        assert_eq!(
            format!("{}", alloc_error),
            "error caused by failed allocation, reason: alloc error"
        );
    }
}

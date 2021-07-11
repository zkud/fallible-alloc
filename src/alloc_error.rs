use std::alloc;
use std::fmt;

#[derive(fmt::Debug)]
pub struct AllocError {
    error_type: AllocErrorType,
    message: String,
}

#[derive(fmt::Debug, Clone, PartialEq)]
pub enum AllocErrorType {
    LayoutError,
    FailedAllocation,
}

impl fmt::Display for AllocErrorType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AllocErrorType::LayoutError => write!(formatter, "LayoutError"),
            AllocErrorType::FailedAllocation => write!(formatter, "FailedAllocation"),
        }
    }
}

impl AllocError {
    pub fn new(message: String, error_type: AllocErrorType) -> AllocError {
        AllocError {
            error_type,
            message,
        }
    }

    pub fn get_message(&self) -> String {
        self.message.clone()
    }

    pub fn get_error_type(&self) -> AllocErrorType {
        self.error_type.clone()
    }
}

impl fmt::Display for AllocError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "alloc error with type: {}, reason: {}",
            self.error_type, self.message
        )
    }
}

impl From<alloc::LayoutError> for AllocError {
    fn from(error: alloc::LayoutError) -> AllocError {
        AllocError::new(format!("{}", error), AllocErrorType::LayoutError)
    }
}

#[cfg(test)]
mod tests {
    use super::AllocError;
    use super::AllocErrorType;

    #[test]
    fn it_inits() {
        let layout_error = AllocError::new("layout error".to_string(), AllocErrorType::LayoutError);

        assert_eq!(layout_error.get_message(), "layout error");
        assert_eq!(layout_error.get_error_type(), AllocErrorType::LayoutError);

        let alloc_error =
            AllocError::new("alloc error".to_string(), AllocErrorType::FailedAllocation);

        assert_eq!(alloc_error.get_message(), "alloc error");
        assert_eq!(
            alloc_error.get_error_type(),
            AllocErrorType::FailedAllocation
        );
    }

    #[test]
    fn it_displayable() {
        let layout_error = AllocError::new("layout error".to_string(), AllocErrorType::LayoutError);

        assert_eq!(
            format!("{}", layout_error),
            "alloc error with type: LayoutError, reason: layout error"
        );

        let alloc_error =
            AllocError::new("alloc error".to_string(), AllocErrorType::FailedAllocation);

        assert_eq!(
            format!("{}", alloc_error),
            "alloc error with type: FailedAllocation, reason: alloc error"
        );
    }
}

//! to do struct
use crate::errors::errors::TravelError;
use crate::models::todo::id::todo_id::TodoId;

#[derive(Debug, Clone)]
pub struct Todo {
    id: TodoId,
    /// This value must be grater than 0 and less than equal 200.
    summary: String,
    /// This value must be grater than 0 and less than equal 500.
    description: Option<String>,
    /// due date. this is time stamp
    due_date: Option<i64>,
    /// This is false by default.
    done: bool
}

impl Todo {
    /// The done is false by default.
    pub fn new(id: &TodoId, summary: &str, description: Option<&str>, due_date: Option<i64>, done: Option<bool>) -> Result<Self, TravelError> {

        if let Err(summary_error) = validate_summary(summary) {
            return Err(summary_error);
        };

        if let Err(description_err) = validate_description(description) {
            return Err(description_err)
        }

        Ok(
            Self {
                id: id.to_owned(),
                summary: summary.to_owned(),
                description: match description {
                    Some(d) => Some(d.to_owned()),
                    None => None
                },
                due_date,
                done: done.unwrap_or_else(|| false)
            }
        )
    }

    pub fn todo_id(&self) -> &TodoId {
        &self.id
    }

    /// update the summary and the description
    pub fn update(&mut self, summary: &str, description: Option<&str>) -> Result<(), TravelError> {

        if let Err(summary_error) = validate_summary(summary) {
            return Err(summary_error);
        };

        if let Err(description_err) = validate_description(description) {
            return Err(description_err)
        }
        self.summary = summary.to_string();
        self.description = match description {
            Some(d) => Some(d.to_string()),
            None => None
        };
        Ok(())
    }
    pub fn toggle_todo(&mut self) {
        self.done = !self.done
    }
}

/// The summary length must be grater than 0 and less than 200.
fn validate_summary(summary: &str) -> Result<(), TravelError> {
    if summary.len() < 1 {
        return Err(TravelError::DomainError("The summary length must be grater than 0.".to_string()))
    };
    if 200 < summary.len() {
        return Err(TravelError::DomainError("The summary length must be less than or equal 200.".to_string()))
    };
    Ok(())
}

fn validate_description(description: Option<&str>) -> Result<(), TravelError> {

    match description {
        Some(desc) => {
            if desc.len() < 1 {
                return Err(TravelError::DomainError("The description length must be grater than 0.".to_string()))
            };
            if 500 < desc.len() {
                return Err(TravelError::DomainError("The description length must be less than or equal 200.".to_string()))
            };
            Ok(())
        },
        None => Ok(())
    }

}


#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn test_new() {
        // Arrange
        let todo_id = TodoId::from(&1u32);

        // Act
        let todo = Todo::new(&todo_id, "summary", None, None, None);

        // Assert
        assert!(todo.is_ok());
    }

    #[test]
    fn test_update() {
        // Arrange
        let todo_id = TodoId::from(&1);
        let mut todo = Todo::new(&todo_id, "summary", None, None, None).unwrap();

        // Act
        let updated = todo.update("updated", Some("updated description"));

        // Assert
        assert!(updated.is_ok());
        assert_eq!(&todo.summary, "updated");
        assert_eq!(&todo.description.unwrap(), "updated description");
    }

    #[test]
    fn test_update_summary_cannot_be_zero() {
        // Arrange
        let todo_id = TodoId::from(&1);
        let mut todo = Todo::new(&todo_id, "summary", None, None, None).unwrap();

        // Act
        let updated = todo.update("", None);

        // Assert
        assert!(updated.is_err());
    }

    #[test]
    fn test_update_summary_cannot_be_grater_than_200() {
        // Arrange
        let todo_id = TodoId::from(&1);
        let mut todo = Todo::new(&todo_id, "summary", None, None, None).unwrap();

        // Act
        let updated = todo.update(&String::from_utf8(vec![b'X'; 201]).unwrap(), None);

        // Assert
        assert!(updated.is_err());
    }

    #[test]
    fn test_update_description_cannot_be_zero() {
        // Arrange
        let todo_id = TodoId::from(&1);
        let mut todo = Todo::new(&todo_id, "summary", None, None, None).unwrap();

        // Act
        let updated = todo.update("summary", Some(""));

        // Assert
        assert!(updated.is_err());
    }

    #[test]
    fn test_update_description_cannot_be_grater_than_500() {
        // Arrange
        let todo_id = TodoId::from(&1);
        let mut todo = Todo::new(&todo_id, "summary", None, None, None).unwrap();

        // Act
        let updated = todo.update("summary", Some(&String::from_utf8(vec![b'X'; 501]).unwrap()));

        // Assert
        assert!(updated.is_err());
    }
}
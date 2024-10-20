//! to do struct
use crate::errors::errors::TravelError;
use crate::models::todo::id::todo_id::TodoId;

#[derive(Debug, Clone, Eq, PartialEq)]
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

    pub fn summary(&self) -> &str {
        &self.summary
    }

    pub fn description(&self) -> Option<&str> {
        match &self.description {
            Some(s) => Some(s),
            None => None
        }
    }
    
    pub fn due_date(&self) -> Option<&i64> {
        match &self.due_date {
            Some(du) => Some(du),
            None => None
        }
    }
    
    pub fn done(&self) -> bool {
        (&self.done).to_owned()
    }

    /// update the summary and the description
    pub fn update(mut self, summary: &str, description: Option<&str>, due_date: Option<i64>) -> Result<Self, TravelError> {

        self.summary = summary.to_string();
        self.description = match description {
            Some(d) => Some(d.to_string()),
            None => None
        };
        self.due_date = match due_date {
            Some(d) => Some(d),
            None => None
        };

        Self::new(&self.id, summary, description, self.due_date, Some(self.done))
    }
    pub fn toggle_todo(mut self) -> Self {
        self.done = !self.done;
        self
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
        let todo = Todo::new(&todo_id, "summary", None, None, None).unwrap();

        // Act
        let updated = todo.update("updated", Some("updated description"), Some(1212121));

        // Assert
        assert!(updated.is_ok());
        let updated_result = updated.expect("updated todo unwrap failed");
        assert_eq!(updated_result.summary(), "updated");
        assert_eq!(updated_result.description().unwrap(), "updated description");
    }

    #[test]
    fn test_update_summary_cannot_be_zero() {
        // Arrange
        let todo_id = TodoId::from(&1);
        let todo = Todo::new(&todo_id, "summary", None, None, None).unwrap();

        // Act
        let updated = todo.update("", None, None);

        // Assert
        assert!(updated.is_err());
    }

    #[test]
    fn test_update_summary_cannot_be_grater_than_200() {
        // Arrange
        let todo_id = TodoId::from(&1);
        let todo = Todo::new(&todo_id, "summary", None, None, None).unwrap();

        // Act
        let updated = todo.update(&String::from_utf8(vec![b'X'; 201]).unwrap(), None, None);

        // Assert
        assert!(updated.is_err());
    }

    #[test]
    fn test_update_description_cannot_be_zero() {
        // Arrange
        let todo_id = TodoId::from(&1);
        let todo = Todo::new(&todo_id, "summary", None, None, None).unwrap();

        // Act
        let updated = todo.update("summary", Some(""), None);

        // Assert
        assert!(updated.is_err());
    }

    #[test]
    fn test_update_description_cannot_be_grater_than_500() {
        // Arrange
        let todo_id = TodoId::from(&1);
        let todo = Todo::new(&todo_id, "summary", None, None, None).unwrap();

        // Act
        let updated = todo.update("summary", Some(&String::from_utf8(vec![b'X'; 501]).unwrap()), None);

        // Assert
        assert!(updated.is_err());
    }
}
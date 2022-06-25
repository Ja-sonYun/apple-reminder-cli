use chrono::NaiveDateTime;
use serde::{de, Deserialize, Deserializer};

const REMINDER_DT_FORMAT: &str = "%b %d, %Y %H:%M";

#[derive(Debug, Clone, PartialEq)]
pub enum DueDate {
    None,
    Date(NaiveDateTime),
}

impl<'de> Deserialize<'de> for DueDate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s == "" {
            Ok(DueDate::None)
        } else {
            let dt = NaiveDateTime::parse_from_str(&s, REMINDER_DT_FORMAT)
                .map_err(|e| de::Error::custom(format!("{}", e)))?;
            Ok(DueDate::Date(dt))
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum IsCompleted {
    YES,
    NO,
}

impl IsCompleted {
    pub fn as_icon(&self) -> String {
        match self {
            IsCompleted::YES => "[x]".to_string(),
            IsCompleted::NO => "[ ]".to_string(),
        }
    }

    pub fn as_string(&self) -> String {
        match self {
            IsCompleted::YES => "Completed".to_string(),
            IsCompleted::NO => "Uncompleted".to_string(),
        }
    }
}

impl<'de> Deserialize<'de> for IsCompleted {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s == "Yes" {
            Ok(IsCompleted::YES)
        } else {
            Ok(IsCompleted::NO)
        }
    }
}

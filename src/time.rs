use std::time::SystemTime;
use std::error::Error;
use std::fmt;
use serde::{Deserialize, Serialize};

pub const CLASS_NUM: usize = 7;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Serialize, Deserialize, Debug)]
pub enum Class {
    Health,
    Physics,
    Econ,
    Stats,
    Calc,
    Chem,
    English,
    Other,
}

impl From<String> for Class {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "health" => Class::Health,
            "physics" => Class::Physics,
            "econ" => Class::Econ,
            "stats" => Class::Stats,
            "calc" => Class::Calc,
            "chem" => Class::Chem,
            "english" => Class::English,
            _ => Class::Other,
        }
    }
}

impl std::fmt::Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Class::Health => write!(f, "Health"),
            Class::Physics => write!(f, "Physics"),
            Class::Econ => write!(f, "Econ"),
            Class::Stats => write!(f, "Stats"),
            Class::Calc => write!(f, "Calc"),
            Class::Chem => write!(f, "Chem"),
            Class::English => write!(f, "English"),
            Class::Other => write!(f, "Other"),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Time {
    start: Option<SystemTime>,
    end: Option<SystemTime>,
    pub class: Class,
}

impl Time {
    pub fn new(class: Class) -> Self {
        Time {
            start: None,
            end: None,
            class,
        }
    }

    pub fn set_start(&mut self) {
        self.start = Some(SystemTime::now());
    }

    pub fn set_end(&mut self) {
        self.end = Some(SystemTime::now());
    }

    pub fn duration(&self) -> Result<chrono::Duration, Box<dyn Error>> {
        match self.start {
            Some(s) => {
                let duration_std = match self.end {
                    Some(e) => e.duration_since(s)?,
                    None => s.elapsed()?,
                };
                Ok(chrono::Duration::from_std(duration_std)?)
            },
            None => Err("No start time")?,
        }
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let duration = match self.duration() {
            Ok(d) => d,
            Err(_) => chrono::Duration::zero(),
        };
        if duration.num_hours() > 0 {
            write!(
                f,
                "{}h {}m {}s",
                duration.num_hours(),
                duration.num_minutes() % 60,
                duration.num_seconds() % 60
                )?;
        } else {
            write!(
                f,
                "{}m {}s",
                duration.num_minutes(),
                duration.num_seconds() % 60
                )?;
        }
        Ok(())
    }
}

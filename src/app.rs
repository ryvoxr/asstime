use chrono;
use serde::{Deserialize, Serialize};
use serde_json;
use std::error::Error;
use std::fs;
use std::io;
use std::path::Path;
use std::time::SystemTime;

pub struct App<'a> {
    path: &'a Path,
    pub times: Vec<Time>,
}

impl<'a> App<'a> {
    pub fn new(path: &'a Path) -> Self {
        App {
            path,
            times: Vec::new(),
        }
    }

    pub fn load_times(&mut self) -> io::Result<()> {
        // create full path if does not exist
        if !self.path.exists() {
            fs::create_dir_all(self.path.parent().ok_or(io::Error::new(
                io::ErrorKind::NotFound,
                "couldn't find parent directory",
            ))?)?;
        }
        match fs::File::open(&self.path) {
            Err(_) => _ = fs::File::create(&self.path)?,
            Ok(_) => (),
        };

        self.times = match serde_json::from_str(&fs::read_to_string(self.path)?) {
            Ok(v) => v,
            Err(_) => Vec::new(),
        };
        Ok(())
    }

    pub fn write_times(&self) -> Result<(), Box<dyn Error>> {
        let serialized = serde_json::to_string(&self.times)?;
        fs::write(self.path, serialized)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Time {
    start: Option<SystemTime>,
    end: Option<SystemTime>,
    class: Class,
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
}

impl std::fmt::Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.start {
            Some(s) => match self.end {
                Some(e) => match e.duration_since(s) {
                    Ok(d) => {
                        let duration = chrono::Duration::from_std(d)
                            .expect("couldn't convert std::time::Duration to chrono::Duration");
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
                    }
                    Err(_) => (),
                },
                None => (),
            },
            None => (),
        };
        Ok(())
    }
}

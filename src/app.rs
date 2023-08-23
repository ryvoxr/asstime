use crate::time::{Class, Time, CLASS_NUM};
use clap::{Args, Parser, Subcommand};
use serde::{Deserialize, Serialize};
use serde_json;
use std::{collections::HashMap, env, error::Error, fs, io, path::Path};

pub fn run(args: Cli) -> Result<(), Box<dyn Error>> {
    let pathstr = env::var("HOME")? + "/.local/share/asstime/times.json";
    let path = Path::new(&pathstr);
    let mut app = App::new(path);
    app.load_data()?;

    match args.command {
        Some(Commands::Start { class }) => {
            app.start_timer(class.into())?;
        }
        Some(Commands::Stop { class }) => {
            app.end_timer(class.into())?;
        }
        Some(Commands::Cancel { class }) => {
            app.cancel_timer(class.into())?;
        }
        Some(Commands::Show(args)) => {
            app.show(args)?;
        }
        None => {
            if args.list_classes {
                app.list_class();
            } else {
                println!("No command given");
            }
        }
    };

    // App cleanup
    app.write_data()?;
    Ok(())
}

struct App<'a> {
    path: &'a Path,
    data: Data,
}

impl<'a> App<'a> {
    fn new(path: &'a Path) -> Self {
        App {
            path,
            data: Data::new(),
        }
    }

    fn start_timer(&mut self, class: Class) -> Result<(), Box<dyn Error>> {
        if self.data.active_times.get(&class).is_some() {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::AlreadyExists,
                "timer already exists",
            )));
        }
        let mut time = Time::new(class);
        time.set_start();
        self.data.active_times.insert(class, time);
        println!("Timer started for {}", class);
        Ok(())
    }

    fn end_timer(&mut self, class: Class) -> Result<(), Box<dyn Error>> {
        match self.data.active_times.get_mut(&class) {
            Some(time) => {
                time.set_end();
                println!("Timer stopped for {} with time {}", class, time);
                self.data.times.push(time.clone());
                self.data.active_times.remove(&class);
            }
            None => {
                return Err(Box::new(io::Error::new(
                    io::ErrorKind::NotFound,
                    "timer not found",
                )));
            }
        }
        Ok(())
    }

    fn cancel_timer(&mut self, class: Class) -> Result<(), Box<dyn Error>> {
        match self.data.active_times.get_mut(&class) {
            Some(_) => {
                self.data.active_times.remove(&class);
            }
            None => {
                return Err(Box::new(io::Error::new(
                    io::ErrorKind::NotFound,
                    "timer not found",
                )));
            }
        }
        println!("Timer canceled for {}", class);
        Ok(())
    }

    fn show(&self, args: ShowArgs) -> Result<(), Box<dyn Error>> {
        match &args.class {
            Some(c) => {
                Ok(self.show_timer(c.to_string().into(), &args))
            }
            None => {
                self.show_timers(args)
            }
        }
    }

    fn show_timer(&self, class: Class, args: &ShowArgs) {
        let n = match args.previous {
            Some(n) => n + 1,
            None => 1,
        };
        let mut found = 0;
        match self.data.active_times.get(&class) {
            Some(time) => {
                println!("{}: {} (active)", class, time);
                found += 1;
            }
            None => (),
        };
        if args.active_only {
            println!("No timer found for {}", class);
            return;
        }
        for time in self.data.times.iter().rev() {
            if n - found <= 0 {
                return;
            }
            if time.class == class {
                println!("{}: {}", class, time);
                found += 1;
            }
        }
        if found == 0 {
            println!("No timer found for {}", class);
        }
    }

    fn show_timers(&self, args: ShowArgs) -> Result<(), Box<dyn Error>> {
        let mut shown_classes: Vec<Class> = Vec::new();
        for (class, time) in &self.data.active_times {
            println!("{}: {} (active)", class, time);
            shown_classes.push(*class);
        }
        if args.active_only {
            return Ok(());
        }
        for time in self.data.times.iter().rev() {
            if shown_classes.len() >= CLASS_NUM {
                break;
            }
            if !shown_classes.contains(&time.class) {
                println!("{}: {}", time.class, time);
                shown_classes.push(time.class);
            }
        }
        if args.sum {
            self.sum()?;
        }
        Ok(())
    }

    fn create_path_if_not_exists(&self) -> io::Result<()> {
        if !self.path.exists() {
            fs::create_dir_all(self.path.parent().ok_or(io::Error::new(
                io::ErrorKind::NotFound,
                "couldn't find parent directory",
            ))?)?;
        }
        match fs::File::open(self.path) {
            Err(_) => _ = fs::File::create(self.path)?,
            Ok(_) => (),
        };
        Ok(())
    }

    fn load_data(&mut self) -> io::Result<()> {
        self.create_path_if_not_exists()?;

        self.data = match serde_json::from_str(&fs::read_to_string(self.path)?) {
            Ok(v) => v,
            Err(_) => Data::new(),
        };
        Ok(())
    }

    fn write_data(&self) -> Result<(), Box<dyn Error>> {
        let serialized = serde_json::to_string(&self.data)?;
        fs::write(self.path, serialized)?;
        Ok(())
    }

    fn list_class(&self) {
        for class in &[
            Class::Health,
            Class::Physics,
            Class::Econ,
            Class::Stats,
            Class::Calc,
            Class::Chem,
            Class::English,
        ] {
            println!("{}", class);
        }
    }

    fn sum(&self) -> Result<(), Box<dyn Error>> {
        let mut used_classes: Vec<Class> = Vec::new();
        let mut duration = chrono::Duration::zero();

        for (class, time) in &self.data.active_times {
            used_classes.push(*class);
            duration = duration + time.duration()?;
        }
        for time in self.data.times.iter().rev() {
            if used_classes.len() >= CLASS_NUM {
                break;
            }
            if !used_classes.contains(&time.class) {
                used_classes.push(time.class);
                duration = duration + time.duration()?;
            }
        }
        if duration.num_hours() > 0 {
            println!(
                "Total: {}h {}m {}s",
                duration.num_hours(),
                duration.num_minutes() % 60,
                duration.num_seconds() % 60
                );
        } else {
            println!(
                "Total: {}m {}s",
                duration.num_minutes(),
                duration.num_seconds() % 60
                );
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
struct Data {
    times: Vec<Time>,
    active_times: HashMap<Class, Time>,
}

impl Data {
    fn new() -> Self {
        Data {
            times: Vec::new(),
            active_times: HashMap::new(),
        }
    }
}

#[derive(Parser)]
#[clap(author, version, about)]
pub struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,

    /// List valid classes
    #[arg(long)]
    list_classes: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Start an assignment timer
    Start { class: String },
    /// Stop an assignment timer
    Stop { class: String },
    /// Cancel an assignment timer
    Cancel { class: String },
    /// Show assignment times
    Show(ShowArgs),
}

#[derive(Args)]
struct ShowArgs {
    /// Show assignment times
    #[arg(short, long)]
    class: Option<String>,
    /// Only show active times
    #[arg(short, long)]
    active_only: bool,
    /// Show N previous classes when class specified
    #[arg(short, long, value_name = "N")]
    previous: Option<i32>,
    /// Sum most recent assignment time of every class
    #[arg(short, long)]
    sum: bool,
}

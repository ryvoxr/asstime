use std::error::Error;
use std::path::Path;
use std::env;

mod app;
use app::*;

fn main() -> Result<(), Box<dyn Error>> {
    let pathstr = env::var("HOME")? + "/.local/share/asstime/times.json";
    let path = Path::new(&pathstr);
    let mut app = App::new(path);
    app.load_times()?;

    for time in &app.times {
        time.print_duration()?;
    }

    app.write_times()?;
    Ok(())
}

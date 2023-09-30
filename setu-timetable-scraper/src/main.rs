use std::error::Error;
use std::process::Command;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(),Box<dyn Error>>{
    // let output = Command::new("geckodriver").arg("-p 9515").output().expect("Failed to execute command");
    // Command::new("killall").arg("geckodriver").spawn().expect("Failed to execute command");
    Command::new("geckodriver").arg("-p").arg("9515").spawn().expect("Failed to execute command");
    setu_timetable::scrape_setu_timetable().await?;
    Command::new("killall").arg("geckodriver").spawn().expect("Failed to execute command");
    Ok(())
}

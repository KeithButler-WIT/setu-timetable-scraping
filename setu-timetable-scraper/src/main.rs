use std::error::Error;
use std::process::Command;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(),Box<dyn Error>>{
    Command::new("killall").arg("geckodriver").spawn().expect("Failed to execute command");
    tokio::time::sleep(Duration::from_secs(1)).await;
    Command::new("geckodriver").arg("-p").arg("9515").spawn().expect("Failed to execute command");
    tokio::time::sleep(Duration::from_secs(2)).await;
    setu_timetable::scrape_setu_timetable().await?;
    Command::new("killall").arg("geckodriver").spawn().expect("Failed to execute command");
    Ok(())
}

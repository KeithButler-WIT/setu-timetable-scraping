use std::error::Error;
use std::process::Command;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(),Box<dyn Error>>{
    check_driver().await;
    kill_driver().await;
    tokio::time::sleep(Duration::from_secs(1)).await;
    start_driver().await;
    tokio::time::sleep(Duration::from_secs(1)).await;

    tokio::time::sleep(Duration::from_secs(1)).await;
    setu_timetable::scrape_setu_timetable().await?;
    tokio::time::sleep(Duration::from_secs(2)).await;
    kill_driver().await;

    Ok(())
}

async fn start_driver() {
    if cfg!(target_os = "windows") {
        Command::new("cmd").args(&["start", "geckodriver.exe"]).spawn().expect("geckodriver not installed");
        Command::new("cmd").args(&["geckodriver", "-p", "9515"]).spawn().expect("Failed to execute command");
    }
    else {
        Command::new("geckodriver").arg("-p").arg("9515").spawn().expect("Failed to execute command");
    }
}

async fn check_driver() {
    if cfg!(target_os = "windows") {
        // TODO
    }
    else {
        Command::new("geckodriver").arg("-V").spawn().expect("geckodriver not installed");
    }
}

async fn kill_driver() {
    if cfg!(target_os = "windows") {
        Command::new("cmd").args(&["tasklist", "/IM", "geckodriver", "/F"]).spawn().expect("Failed to execute command");
    }
    else {
        Command::new("killall").arg("geckodriver").spawn().expect("Failed to execute command");
    }
}
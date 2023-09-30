use std::error::Error;

#[tokio::main]
async fn main() -> Result<(),Box<dyn Error>>{
    setu_timetable::scrape_setu_timetable().await?;
    Ok(())
}

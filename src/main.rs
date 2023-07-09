mod api;
mod file_parser;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let request_sender = api::RequestSender::new("http://worldtimeapi.org/api/timezone/");

    let lines = file_parser::parse_file("zones.txt")?;
    for line in lines {
        let resp = request_sender.make_request(&line).await?;
        println!("{}", resp);
    }
    Ok(())
}

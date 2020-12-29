use jieba_server::*;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("./xxx 0.0.0.0:8081");
    }

    let addr = args[1].parse()?;
    let ser = server::JiebaServer::default();
    println!("Jieba Server Run {}", &addr);
    Server::builder().
        add_service(SingleJiebaServer::new(ser)).
        serve(addr).
        await?;

    Ok(())
}

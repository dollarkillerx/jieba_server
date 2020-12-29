use jieba_server::*;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("./xxx 0.0.0.0:8081  poolSize");
    }

    let addr = args[1].parse()?;
    let ser = server::JiebaServer::default();
    jieba::global_data(args[2].parse()?);
    println!("Jieba Server Run {}", &addr);
    Server::builder().
        add_service(SingleJiebaServer::new(ser)).
        serve(addr).
        await?;

    Ok(())
}

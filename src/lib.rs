pub mod server;
pub mod jieba;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

use tonic;
pub use jieba_server_proto::single_jieba_server::SingleJiebaServer;
pub use tonic::{transport::Server, Request, Response, Status, Code};

pub mod jieba_server_proto {
    tonic::include_proto!("single_jieba");
}
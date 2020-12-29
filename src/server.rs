use super::*;
use jieba_server_proto::single_jieba_server::SingleJieba;
use jieba_server_proto::{CutReq, CutResp};
use jieba_rs::Jieba;
use once_cell::sync::OnceCell;

#[derive(Debug, Default)]
pub struct JiebaServer {}

const DEFAULT_DICT: &'static str = "data/dict.txt";

fn global_jieba() -> &'static Jieba {
    static JIEBA: OnceCell<Jieba> = OnceCell::new();
    JIEBA.get_or_init(|| {
        use std::io::BufReader;
        let mut p = Jieba::new();
        let mut default_dict = BufReader::new(DEFAULT_DICT.as_bytes());
        p.load_dict(&mut default_dict).unwrap();
        let p = p;  // 去掉mut
        p
    })
}

#[tonic::async_trait]
impl SingleJieba for JiebaServer {
    async fn cut(&self, request: Request<CutReq>) -> std::result::Result<Response<CutResp>, Status> {
        let req: CutReq = request.into_inner();

        let jieba = global_jieba();
        let c = jieba.cut_all(req.message.as_str());
        let mut response = CutResp {
            word: Vec::new(),
        };
        for i in c {
            response.word.push(i.to_string());
        }

        Ok(Response::new(response))
    }
}

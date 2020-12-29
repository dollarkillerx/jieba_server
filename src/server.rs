use super::*;
use jieba_server_proto::single_jieba_server::SingleJieba;
use jieba_server_proto::{CutReq, CutResp};

#[derive(Debug, Default)]
pub struct JiebaServer {}

#[tonic::async_trait]
impl SingleJieba for JiebaServer {
    async fn cut(&self, request: Request<CutReq>) -> std::result::Result<Response<CutResp>, Status> {
        let req: CutReq = request.into_inner();

        let response = CutResp {
            word: vec!["str".to_string()],
        };

        Ok(Response::new(response))
    }
}

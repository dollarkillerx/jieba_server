use super::*;
use jieba_server_proto::single_jieba_server::SingleJieba;
use jieba_server_proto::{CutReq, CutResp};

#[derive(Default)]
pub struct JiebaServer {}

#[tonic::async_trait]
impl SingleJieba for JiebaServer {
    async fn cut(&self, request: Request<CutReq>) -> std::result::Result<Response<CutResp>, Status> {
        let req: CutReq = request.into_inner();
        let r = jieba::global_data(0);
        let jieba = r.get().await.map_err(|err| {
            Status::new(Code::Internal, format!("{}",err))
        })?;

        let c = jieba.cut_all(req.message.as_str());
        r.put(jieba).await.map_err(|err| {
            Status::new(Code::Internal, format!("{}",err))
        })?;
        let mut response = CutResp {
            word: Vec::new(),
        };
        for i in c {
            response.word.push(i.to_string());
        }

        Ok(Response::new(response))
    }
}

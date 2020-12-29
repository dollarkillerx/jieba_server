use super::*;
use tokio::sync::mpsc;
use jieba_rs::Jieba;
use tokio::sync::mpsc::{Sender, Receiver};
use tokio::sync::Mutex;
use once_cell::sync::OnceCell;

pub fn global_data(pool_size: usize) -> &'static JiebaPool {
    static POOL: OnceCell<JiebaPool> = OnceCell::new();
    POOL.get_or_init(|| {
        let pool = JiebaPool::new(pool_size);
        pool
    })
}

const DEFAULT_DICT: &'static str = "data/dict.txt";

/// jieba pool
pub struct JiebaPool {
    pool_sen: Mutex<Sender<Jieba>>,
    pool_rec: Mutex<Receiver<Jieba>>,
}

impl JiebaPool {
    pub fn new(mut size: usize) -> JiebaPool {
        if size == 0 {
            size = 10
        }

        let (sen, rec) = mpsc::channel(size);
        let mut p = sen.clone();
        tokio_test::block_on(async {
            for _ in 0..size {
                p.send({
                    use std::io::BufReader;
                    let mut p = Jieba::new();
                    let mut default_dict = BufReader::new(DEFAULT_DICT.as_bytes());
                    p.load_dict(&mut default_dict).unwrap();
                    let p = p;  // 去掉mut
                    p
                }).await.unwrap();
            }
        });

        JiebaPool {
            pool_rec: Mutex::new(rec),
            pool_sen: Mutex::new(sen),
        }
    }

    pub async fn get(&self) -> Result<Jieba> {
        let mut pool = self.pool_rec.lock().await;
        return match (*pool).recv().await {
            None => {
                Err("recv err".into())
            }
            Some(o) => {
                Ok(o)
            }
        };
    }

    pub async fn put(&self, jieba: Jieba) -> Result<()> {
        let mut pool = self.pool_sen.lock().await;
        (&mut *pool).send(jieba).await.map_err(|_| {
            "send".into()
        })
    }
}
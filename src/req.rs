use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::time;

pub static UA: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/102.0.0.0 Safari/537.36";

pub(crate) struct QBQuery {
    client: reqwest::Client,
}

impl QBQuery {
    pub async fn new() -> anyhow::Result<QBQuery> {
        let client = reqwest::Client::builder()
            .user_agent(UA)
            .pool_idle_timeout(time::Duration::from_secs(50))
            .connect_timeout(time::Duration::from_secs(10))
            .timeout(time::Duration::from_secs(30))
            .build()?;
        Ok(Self { client })
    }

    //noinspection DuplicatedCode
    async fn extract_redirect_url(&self, content: String) -> anyhow::Result<String> {
        let document = nipper::Document::from(content.as_str());
        let content = document.select("script");
        let text = String::from(content.text());
        let path = text
            .trim()
            .split("window.location.href =\"")
            .filter(|x| x.contains("/"))
            .map(|x| {
                x.split("\";")
                    .filter(|x| x.contains("/"))
                    .collect::<String>()
            })
            .collect::<String>();
        Ok(format!("https://qb-api.ltd{}", path))
    }

    async fn request_handler(&self, url: String) -> anyhow::Result<String> {
        // let mut t = tokio::spawn(async move {
        //     loop {
        //         for c in "-\\|/".chars() {
        //             print!("\r{}", c);
        //             std::io::stdout().flush().expect("stdout flush panic!");
        //             std::thread::sleep(std::time::Duration::from_millis(100))
        //         }
        //     }
        // });
        let res = self.client.get(url).send().await?.text().await?;
        Ok(res)
    }

    async fn remove_html(&self, mut res: String) -> anyhow::Result<String> {
        if res.contains("<br />") {
            let offset = res
                .find("{")
                .ok_or(anyhow::anyhow!("find json boundary errors"))?;
            res.replace_range(..offset, "");
        }
        Ok(res)
    }

    async fn body_handler(&self, res: String) -> anyhow::Result<DataResult> {
        if res.contains("html") {
            let url = self.extract_redirect_url(res).await?;
            let res = self.request_handler(url).await?;
            let handler_res = self.remove_html(res).await?;
            let result = serde_json::from_str::<DataResult>(handler_res.as_ref());
            return match result {
                Ok(res) => Ok(res),
                Err(err) => Err(anyhow!(err)),
            };
        }
        let handler_res = self.remove_html(res).await?;
        let result = serde_json::from_str::<DataResult>(handler_res.as_ref());
        return match result {
            Ok(res) => Ok(res),
            Err(err) => Err(anyhow!(err)),
        };
    }

    // qq查询
    pub async fn query_qq_for_qq(&self, qq: &str) -> anyhow::Result<DataResult> {
        let url = format!("https://qb-api.ltd/allcha.php?qq={}", qq);
        let res = self.request_handler(url).await?;
        self.body_handler(res).await
    }

    // 反查qq
    pub async fn reverse_query_qq_for_mobile(&self, mobile: &str) -> anyhow::Result<DataResult> {
        let url = format!("https://qb-api.ltd/mob-api.php?mod=cha&hm={}", mobile);
        let res = self.request_handler(url).await?;
        self.body_handler(res).await
    }

    // 16e qq查询
    pub async fn query_16e_qq_for_qq(&self, qq: &str) -> anyhow::Result<DataResult> {
        let url = format!("https://qb-api.ltd/16e-api.php?mod=cha&qq={}", qq);
        let res = self.request_handler(url).await?;
        self.body_handler(res).await
    }

    // 微博查询
    pub async fn query_weibo_for_uid(&self, uid: &str) -> anyhow::Result<DataResult> {
        let url = format!("https://qb-api.ltd/wb-api.php?mod=cha&uid={}", uid);
        let res = self.request_handler(url).await?;
        self.body_handler(res).await
    }

    // 微博反查
    pub async fn reverse_query_weibo_for_mobile(&self, mobile: &str) -> anyhow::Result<DataResult> {
        let url = format!("https://qb-api.ltd/wb-fc.php?mod=cha&hm={}", mobile);
        let res = self.request_handler(url).await?;
        self.body_handler(res).await
    }

    // lol查询
    pub async fn query_lol_for_uid(&self, qq: &str) -> anyhow::Result<DataResult> {
        let url = format!("https://qb-api.ltd/lol-api.php?mod=cha&uin={}", qq);
        let res = self.request_handler(url).await?;
        self.body_handler(res).await
    }

    // lol反查
    pub async fn reverse_query_lol_for_name(&self, name: &str) -> anyhow::Result<DataResult> {
        let url = format!("https://qb-api.ltd/lol-fc.php?mod=cha&name={}", name);
        let res = self.request_handler(url).await?;
        self.body_handler(res).await
    }
}

#[derive(Serialize, Deserialize, Default)]
pub(crate) struct DataResult {
    pub(crate) code: u8,
    pub(crate) msg: Option<String>,
    pub(crate) qq: Option<String>,
    pub(crate) data: Option<Data>,
    pub(crate) place: Option<String>,
}

#[derive(Serialize, Deserialize, Default)]
pub(crate) struct Data {
    pub(crate) qq: Option<String>,
    pub(crate) mobile: Option<String>,
    pub(crate) name: Option<String>,
    pub(crate) dq: Option<String>,
    pub(crate) uid: Option<String>,
    pub(crate) place: Option<String>,
    pub(crate) wb: Option<String>,
    pub(crate) lol: Option<LOL>,
}

#[derive(Serialize, Deserialize, Default)]
pub(crate) struct LOL {
    pub(crate) qq: Option<String>,
    pub(crate) name: Option<String>,
    pub(crate) area: Option<String>,
    pub(crate) dq: Option<String>,
}

use anyhow::anyhow;
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;
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
    async fn extract_redirect_url(&self, mut content: String) -> anyhow::Result<String> {
        let offset = content
            .find("window.location.href")
            .ok_or(anyhow::anyhow!("find script boundary errors")).unwrap();
        content.replace_range(..offset, "");
        let path = content.replace("</script>", "")
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

    pub async fn query_qq_for_qq(&self, qq: &str) -> anyhow::Result<DataResult> {
        let url = format!("https://qb-api.ltd/allcha.php?qq={}", qq);
        let res = self.request_handler(url).await?;
        self.body_handler(res).await
    }

    pub async fn reverse_query_qq_for_mobile(&self, mobile: &str) -> anyhow::Result<DataResult> {
        let url = format!("https://qb-api.ltd/mob-api.php?mod=cha&hm={}", mobile);
        let res = self.request_handler(url).await?;
        self.body_handler(res).await
    }

    pub async fn query_16e_qq_for_qq(&self, qq: &str) -> anyhow::Result<DataResult> {
        let url = format!("https://qb-api.ltd/16e-api.php?mod=cha&qq={}", qq);
        let res = self.request_handler(url).await?;
        self.body_handler(res).await
    }

    pub async fn query_weibo_for_uid(&self, uid: &str) -> anyhow::Result<DataResult> {
        let url = format!("https://qb-api.ltd/wb-api.php?mod=cha&uid={}", uid);
        let res = self.request_handler(url).await?;
        self.body_handler(res).await
    }

    pub async fn reverse_query_weibo_for_mobile(&self, mobile: &str) -> anyhow::Result<DataResult> {
        let url = format!("https://qb-api.ltd/wb-fc.php?mod=cha&hm={}", mobile);
        let res = self.request_handler(url).await?;
        self.body_handler(res).await
    }

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

#[derive(Serialize, Default)]
pub(crate) struct LOL {
    pub(crate) dq: Option<String>,
    pub(crate) qq: Option<String>,
    pub(crate) name: Option<String>,
    pub(crate) area: Option<String>,
}

// device Deserialize error will occur
impl<'de> Deserialize<'de> for LOL {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // this is error
        // let result = String::deserialize(deserializer);
        let res: Result<HashMap<String, String>, D::Error> = HashMap::deserialize(deserializer);
        return match res {
            Ok(map) => {
                let empty_str = String::new();
                let dq = map.get("dq").unwrap_or(&empty_str);
                let qq = map.get("qq").unwrap_or(&empty_str);
                let name = map.get("name").unwrap_or(&empty_str);
                let area = map.get("area").unwrap_or(&empty_str);
                let lol = LOL {
                    dq: Some(dq.to_string()),
                    qq: Some(qq.to_string()),
                    name: Some(name.to_string()),
                    area: Some(area.to_string()),
                };
                Ok(lol)
            }
            Err(_) => Ok(LOL::default()),
        };
    }
}

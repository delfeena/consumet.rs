use crate::models::{ISubtitle, IVideo};
use crate::utils::util_funcs::USER_AGENT;
use reqwest::Url;
use std::collections::HashMap;

pub struct VidCloud {
    pub sources: Vec<IVideo>,
    pub subtitles: Vec<ISubtitle>,
}

const HOST: &str = "https://dokicloud.one";
const HOST2: &str = "https://rabbitstream.net";

impl VidCloud {
    const SERVER_NAME: &'static str = "VidCloud";

    async fn extract(
        &mut self,
        video_url: Url,
        is_alternative: Option<bool>,
    ) -> anyhow::Result<VidCloud> {
        let is_alternative = is_alternative.unwrap_or(false);

        let client = reqwest::Client::new();

        let host = if !is_alternative { HOST } else { HOST2 };

        let _res = client
            .get(format!("{}/ajax/embed-4/getSources?id={}", host, video_url))
            .header("X-Requested-With", "XMLHttpRequest")
            .header("Referer", video_url.to_string())
            .header("User-Agent", USER_AGENT)
            .send()
            .await?;

        self.sources.push(IVideo {
            url: String::new(),
            quality: None,
            is_m3u8: None,
            is_dash: None,
            size: None,
            other: { HashMap::new() },
        });

        self.subtitles.push(ISubtitle {
            id: None,
            url: None,
            lang: None,
        });

        Ok(VidCloud {
            sources: self.sources.clone(),
            subtitles: self.subtitles.clone(),
        })
    }
}

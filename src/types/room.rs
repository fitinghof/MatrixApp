use matrix_sdk::reqwest::Url;
use matrix_sdk::{self, media::MediaRequestParameters};

pub struct Room {
    icon: Option<Url>,
}

impl Room {
    async fn from(value: matrix_sdk::Room) -> Self {
        // let icon = value.avatar_url();

        // if let Some(icon) = icon {
        //     let params =
        //     value
        //         .client()
        //         .media()
        //         .get_media_content(, true);
        // }

        Self {
            icon: Some(Url::parse("https://example.net").unwrap()),
        }
    }
}

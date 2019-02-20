use crate::models::anilist::user::MediaConnection;
use crate::commands::anilist::utils::synopsis;

#[derive(Deserialize, Debug)]
pub struct CharacterName {
    pub first: Option<String>,
    pub last: Option<String>,
    pub native: Option<String>,
    pub alternative: Vec<String>
}

#[derive(Deserialize, Debug)]
pub struct CharacterImage {
    pub large: Option<String>,
    pub medium: Option<String>
}

#[derive(Deserialize, Debug)]
pub struct CharacterBase {
    pub id: u32,

    #[serde(rename = "siteUrl")]
    pub site_url: String,

    pub name: CharacterName
}

impl CharacterBase {
    pub fn full_name(&self) -> String {
        let mut name_list = vec![];

        match &self.name.first {
            Some(first) => name_list.push(first.clone()),
            None => {},
        }

        match &self.name.last {
            Some(last) => name_list.push(last.clone()),
            None => {},
        }

        name_list.join(" ")
    }
}

#[derive(Deserialize, Debug)]
pub struct Character {
    pub id: u32,

    #[serde(rename = "siteUrl")]
    pub site_url: String,

    pub description: Option<String>,

    pub name: CharacterName,

    pub image: CharacterImage,

    pub media: MediaConnection
}

impl Character {
    pub fn full_name(&self) -> String {
        let mut name_list = vec![];

        match &self.name.first {
            Some(first) => name_list.push(first.clone()),
            None => {},
        }

        match &self.name.last {
            Some(last) => name_list.push(last.clone()),
            None => {},
        }

        name_list.join(" ")
    }

    pub fn about(&self) -> String {
        match &self.description {
            Some(description) => synopsis(description, 300),
            None => String::new()
        }
    }

    pub fn cover_image(&self) -> String {
        match &self.image.large {
            Some(image) => format!("{}", image),
            None => String::new()
        }
    }

    pub fn media_list(&self, media_type: &str) -> String {
        let media_list = &self.media.nodes;

        let mut fav_list: Vec<String> = vec![];

        if media_list.len() > 0 {
            let mut count = 0;
            for media in media_list {
                if media.media_type == media_type {
                    fav_list.push(
                        format!("[{}]({})", media.title.user_preferred, media.site_url));
                    count = count + 1;
                }
                if count >= 5 { break }
            }
        }

        if fav_list.len() > 0 {
            return fav_list.join("\n");
        }

        return String::from("N/A")
    }
}
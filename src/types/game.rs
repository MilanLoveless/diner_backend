use super::{game_description::GameDescription, game_link::GameLink, game_name::GameName};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct GameFormData {
    pub name: String,
    pub description: String,
    pub link: String,
}

pub struct GameData {
    pub name: GameName,
    pub description: GameDescription,
    pub link: GameLink,
}

impl TryFrom<GameFormData> for GameData {
    type Error = String;
    fn try_from(value: GameFormData) -> Result<Self, Self::Error> {
        let name = GameName::parse(value.name)?;
        let description = GameDescription::parse(value.description)?;
        let link = GameLink::parse(value.link)?;
        Ok(Self {
            name,
            description,
            link,
        })
    }
}

#[derive(Serialize)]
pub struct GameRecord {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub link: String,
}

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct UserInput {
    pub image : String,
    pub style: String,
    pub furniture: String,
    pub color: String,
}
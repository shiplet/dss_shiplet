pub use serde::{Deserialize, Serialize};
pub use serde_json::{Result, Value};

#[derive(Serialize, Deserialize, Debug)]
pub struct DSSData {
	pub data: Data
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
	pub StandardCollection: StandardCollection
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StandardCollection {
	pub containers: Vec<Container>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Container {
	pub set: Set
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Set {
	pub items: Option<Vec<Item>>,
	pub text: TextTitle,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
	pub image: Image,
	pub text: Text,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Image {
	pub tile: Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Text {
	pub title: TextTitle
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TextTitle {
	pub slug: Option<TitleType>,
	pub full: Option<TitleType>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TitleType {
	pub series: Option<TitleDefault>,
	pub program: Option<TitleDefault>,
	pub set: Option<TitleDefault>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TitleDefault {
	pub default: TitleDefaultInner
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct TitleDefaultInner {
	pub content: String,
	pub language: String,
	pub sourceEntity: String,
}

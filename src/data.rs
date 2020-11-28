use crate::DSSData;
use std::error::Error;
use reqwest::{Error as rqErr};

pub fn prepare_data() -> Result<DSSData, Box<dyn Error>> {
	let body = get_data()?;
	let data_initial: serde_json::error::Result<DSSData> = serde_json::from_str(body.as_str());
	match data_initial {
		Ok(dss) => Ok(dss),
		Err(e) => Err(Box::new(e))
	}
}

fn get_data() -> Result<String, rqErr> {
	let data = reqwest::blocking::get("https://cd-static.bamgrid.com/dp-117731241344/home.json");
	match data {
		Ok(dr) => match dr.text() {
			Ok(ds) => Ok(ds),
			Err(e) => Err(e)
		}
		Err(e) => Err(e)
	}
}
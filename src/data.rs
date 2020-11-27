use crate::DSSData;
use std::error::Error;
use reqwest::{Error as rqErr};

pub async fn prepare_data() -> Result<DSSData, Box<dyn Error>> {
    let body = get_data().await?;
    let data_initial: serde_json::error::Result<DSSData> = serde_json::from_str(body.as_str());
    match data_initial {
        Ok(dss) => Ok(dss),
        Err(e) => Err(Box::new(e))
    }
}

async fn get_data() -> Result<String, rqErr> {
    let data = reqwest::get("https://cd-static.bamgrid.com/dp-117731241344/home.json").await;
    match data {
        Ok(dr) => match dr.text().await {
            Ok(ds) => Ok(ds),
            Err(e) => Err(e)
        }
        Err(e) => Err(e)
    }
}
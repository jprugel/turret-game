use bevy::{
    asset::{AssetLoader, LoadContext, io::Reader},
    prelude::*,
    reflect::TypePath,
};
use serde::Deserialize;
use serde_json5;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Deserialize)]
enum Json5Value {
    String(String),
    Number(f64),
    Boolean(bool),
    Array(Vec<Json5Value>),
    Object(HashMap<String, Json5Value>),
}

#[derive(Asset, TypePath, Debug, Deserialize)]
pub struct Json5Asset {
    data: Option<HashMap<String, Json5Value>>,
}

#[derive(Default)]
pub struct Json5Loader;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum Json5LoaderError {
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),

    #[error("Could not parse JSON5: {0}")]
    Json5(#[from] serde_json5::Error),
}

impl AssetLoader for Json5Loader {
    type Asset = Json5Asset;
    type Settings = ();
    type Error = Json5LoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        _ctx: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut data = Vec::new();
        reader
            .read_to_end(&mut data)
            .await
            .map_err(|err| Json5LoaderError::Io(err))?;

        let parsed = serde_json5::from_slice(&data).map_err(|err| Json5LoaderError::Json5(err))?;

        Ok(Json5Asset { data: Some(parsed) })
    }

    fn extensions(&self) -> &[&str] {
        &["json5"]
    }
}

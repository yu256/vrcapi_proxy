use anyhow::Result;
use dirs_2::home_dir;
use serde::{de::DeserializeOwned, Serialize};
use std::{
    collections::HashMap,
    fs::{create_dir_all, File},
    io::{BufReader, BufWriter, Read, Write},
    path::PathBuf,
    sync::LazyLock,
};

pub(crate) static DATA_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| home_dir().unwrap().join("vrcapi_proxy"));

pub fn read_json<T>(path: &str) -> Result<T>
where
    T: DeserializeOwned,
{
    let mut file = BufReader::new(File::open(DATA_PATH.join(path))?);

    let mut content = String::new();

    file.read_to_string(&mut content)?;

    serde_json::from_str(&content).map_err(From::from)
}

pub fn write_json<T>(data: &T, name: &str) -> Result<()>
where
    T: Serialize,
{
    let Ok(file) = File::create(DATA_PATH.join(format!("{}.json", name))) else {
        create_dir_all(&*DATA_PATH)?;
        return write_json(data, name);
    };

    let json = serde_json::to_string(&data)?;

    let mut file = BufWriter::new(file);

    file.write_all(json.as_bytes()).map_err(From::from)
}

pub(crate) trait HashMapExt {
    fn add(&mut self, auth: &str, token: &str) -> Result<()>;
}

impl HashMapExt for HashMap<String, String> {
    fn add(&mut self, auth: &str, token: &str) -> Result<()> {
        self.insert(auth.to_owned(), token.to_owned());
        write_json(self, "data")
    }
}

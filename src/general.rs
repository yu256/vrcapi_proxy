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
    LazyLock::new(|| home_dir().expect("ホームディレクトリの取得に失敗しました。").join("vrcapi_proxy"));

pub(crate) fn read_json<T>(path: &str) -> Result<T>
where
    T: DeserializeOwned,
{
    let mut file = BufReader::new(File::open(DATA_PATH.join(path))?);

    let mut content = String::new();

    file.read_to_string(&mut content)?;

    serde_json::from_str(&content).map_err(From::from)
}

pub(crate) fn write_json<T>(data: &T, name: &str) -> Result<()>
where
    T: Serialize,
{
    let Ok(file) = File::create(DATA_PATH.join(format!("{}.json", name))) else {
        create_dir_all(&*DATA_PATH)?;
        return write_json(data, name);
    };

    let json = serde_json::to_string(data)?;

    let mut file = BufWriter::new(file);

    file.write_all(json.as_bytes()).map_err(From::from)
}

pub(crate) fn return_not_empty<T>(s1: T, s2: T, s3: T) -> String
where
    T: Into<String> + AsRef<str>,
{
    if !s1.as_ref().is_empty() {
        s1.into()
    } else if !s2.as_ref().is_empty() {
        s2.into()
    } else {
        s3.into()
    }
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

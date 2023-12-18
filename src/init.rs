use crate::{
    general::{read_json, write_json},
    global::DATA_PATH,
};
use anyhow::{ensure, Result};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::remove_file, io};

#[derive(Serialize, Deserialize, Default)]
pub(super) struct Data {
    pub(super) listen: String,
    pub(super) cors: String,
    pub(super) auth: String,
    pub(super) token: String,
}

pub(super) fn init() -> Result<()> {
    #[derive(Serialize, Deserialize)]
    struct Config {
        listen: String,
        cors: String,
    }

    impl Default for Config {
        fn default() -> Self {
            Self {
                listen: "0.0.0.0:8000".into(),
                cors: "http://localhost:3000".into(),
            }
        }
    }

    if let Ok(data) = read_json::<Data>("data.json") {
        ensure!(
            !data.auth.is_empty(),
            "認証IDが空です。入力して再度起動してください。"
        );
        return Ok(());
    }

    let old_config = {
        #[derive(Deserialize)]
        struct OldCorsConfig {
            url: String,
        }
        if let Ok(old_config) = read_json::<OldCorsConfig>("config.json") {
            Config {
                cors: old_config.url,
                ..Default::default()
            }
        } else {
            read_json::<Config>("config.json").unwrap_or_default()
        }
    };

    let data = if let Ok(old_data) = read_json::<HashMap<String, String>>("data.json") {
        let fmt = old_data
            .iter()
            .enumerate()
            .map(|(index, (auth, token))| format!("{index}: {auth} {token}"))
            .reduce(|acc, val| acc + "\n" + &val);

        if let Some(fmt) = fmt {
            println!("マイグレーションする認証情報を選択してください。\n{fmt}");
            let old_data = {
                let mut buffer = String::new();
                loop {
                    io::stdin().read_line(&mut buffer)?;
                    match buffer.trim().parse::<usize>() {
                        Ok(index) => match old_data.iter().enumerate().find(|data| data.0 == index)
                        {
                            Some(data) => break data.1,
                            None => {
                                eprintln!("{index}は存在しません。");
                                buffer.clear();
                            }
                        },
                        Err(e) => {
                            eprintln!("{e}");
                            buffer.clear();
                        }
                    }
                }
            };

            Data {
                listen: old_config.listen,
                cors: old_config.cors,
                auth: old_data.0.into(),
                token: old_data.1.into(),
            }
        } else {
            Data {
                listen: old_config.listen,
                cors: old_config.cors,
                ..Default::default()
            }
        }
    } else {
        Data {
            listen: old_config.listen,
            cors: old_config.cors,
            ..Default::default()
        }
    };

    write_json(&data, "data.json")?;
    remove_file(DATA_PATH.join("config.json"))?;

    println!(
        "{}にコンフィグファイルを生成しました。",
        DATA_PATH.display()
    );

    std::process::exit(0);
}

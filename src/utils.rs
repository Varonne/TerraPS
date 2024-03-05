use reqwest::get;
use serde::Serialize;
use serde_json::{from_reader, ser::PrettyFormatter, to_writer_pretty, Result, Serializer, Value};
use std::{
    fs::{DirBuilder, File},
    io::BufReader,
    path::Path,
};

pub fn update_data(url: &str) {
    const BASE_URL_LIST: [(&str, &str); 2] = [
        ("https://raw.githubusercontent.com/Kengxxiao/ArknightsGameData/master/zh_CN/gamedata", "./data"),
        // ("https://raw.githubusercontent.com/Kengxxiao/ArknightsGameData/master/en_US/gamedata", "./data"),
        ("https://ak-conf.hypergryph.com/config/prod/announce_meta/Android", "./data/announce"),
        // ("https://ark-us-static-online.yo-star.com/announce/Android", "./data/announce"),
    ];

    let local_path: &str = match BASE_URL_LIST.iter().find(|(base_url, _)| url.contains(base_url)) {
        Some((_, path)) => path,
        None => url,
    };

    if url.contains("Android/version") {}
}

pub fn read_json(path: &str) -> Result<Value> {
    let json_reader = BufReader::new(File::open(path).unwrap());
    from_reader(json_reader)
}

pub fn write_json(path: &str, value: Value) -> Result<()> {
    let file = File::create(path).unwrap();
    let fmt = PrettyFormatter::with_indent(b"   ");
    let mut buf = Vec::new();
    let mut ser = Serializer::with_formatter(&mut buf, fmt);
    value.serialize(&mut ser).unwrap();
    to_writer_pretty(file, &value)
}

pub fn decrypt_battle_data() {
    const LOG_TOKEN_KEY: &str = "pM6Umv*^hVQuB6t&";
}
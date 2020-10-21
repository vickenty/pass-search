use std::io::ErrorKind;

use serde::{
    de::{Error, MapAccess, Visitor},
    Deserializer,
};

#[derive(Debug, Clone)]
pub struct Conf {
    pub copy_cmd: Vec<String>,
}

impl Default for Conf {
    fn default() -> Self {
        Conf {
            copy_cmd: vec!["pass".into(), "show".into(), "-c".into()],
        }
    }
}

impl<'de> serde::de::Deserialize<'de> for Conf {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(ConfVisitor)
    }
}

struct ConfVisitor;

impl<'de> Visitor<'de> for ConfVisitor {
    type Value = Conf;

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut conf = Conf::default();

        while let Some(k) = map.next_key::<&str>()? {
            if k == "copy_cmd" {
                conf.copy_cmd = map.next_value()?;
                if conf.copy_cmd.is_empty() {
                    return Err(Error::custom("copy_cmd can't be empty"));
                }
            }
        }

        Ok(conf)
    }

    fn expecting(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str("map")
    }
}

pub fn load() -> anyhow::Result<Conf> {
    let dirs = xdg::BaseDirectories::new()?;
    let path = dirs.place_config_file("pass-search.toml")?;
    let data = match std::fs::read(path) {
        Ok(data) => data,
        Err(e) if e.kind() == ErrorKind::NotFound => {
            return Ok(Conf::default());
        }
        Err(e) => return Err(e.into()),
    };
    Ok(toml::from_slice(&data)?)
}

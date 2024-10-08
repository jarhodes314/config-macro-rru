//! # Handcrafted
//! 
//! To give you some intutition for what the macros generate, without forcing you
//! to touch proc-macro code, this exercise contains the macro code generated by
//! hand.
//! 
//! The code demonstrates (a reasonable, but simplified chunk of) what define_tedge_config!
//! generates before the multi-value changes. This exercise involves first changing the `c8y`
//! field in the DTO and reader structs to be a "multi" field. Once you've done that, you should
//! be able to make the tests pass by changing some of the other code in the file.
//! 
//! NB A couple of the tests contain `todo!`s, you'll need to fill them out once you've started on
//! the code changes. 

use std::{borrow::Cow, fmt, str::FromStr};

use anyhow::bail;
use serde::{Deserialize, Serialize};

mod multi;

/// The DTO - what we deserialise tedge.toml into
#[derive(Debug, Deserialize, Serialize)]
struct TEdgeConfigDto {
    #[serde(default)]
    device: TEdgeConfigDtoDevice,
    // TODO replace me with `Multi<TEdgeConfigDtoC8y>`
    c8y: TEdgeConfigDtoC8y,
    #[serde(default)]
    mqtt: TEdgeConfigDtoMqtt,
}

/// The reader, what tedge.toml gets converted to before anything in thin-edge reads the configuration
pub struct TEdgeConfigReader {
    pub device: TEdgeConfigReaderDevice,
    // TODO replace me with `Multi<TEdgeConfigDtoC8y>`
    pub c8y: TEdgeConfigReaderC8y,
    pub mqtt: TEdgeConfigReaderMqtt,
}

pub struct TEdgeConfigReaderDevice {
    pub id: String,
    pub ty: String,
}

pub struct TEdgeConfigReaderC8y {
    pub url: String,
}

pub struct TEdgeConfigReaderMqtt {
    pub port: u16,
}

#[derive(Debug, Deserialize, Serialize)]
struct TEdgeConfigDtoDevice {
    ty: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct TEdgeConfigDtoC8y {
    url: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct TEdgeConfigDtoMqtt {
    port: u16,
}


impl From<TEdgeConfigDto> for TEdgeConfigReader {
    fn from(value: TEdgeConfigDto) -> Self {
        Self {
            device: value.device.into(),
            c8y: value.c8y.into(),
            mqtt: value.mqtt.into(),
        }
    }
}

impl From<TEdgeConfigDtoDevice> for  TEdgeConfigReaderDevice {
    fn from(value: TEdgeConfigDtoDevice) -> Self {
        Self {
            // In practice we have a complicated way of reading this lazily
            // But we'll ignore that for the sake of simplicity here
            id: "some-device".into(),
            ty: value.ty,
        }
    }
}

impl From<TEdgeConfigDtoC8y> for TEdgeConfigReaderC8y {
    fn from(value: TEdgeConfigDtoC8y) -> Self {
        Self {
            url: value.url,
        }
    }
}

impl From<TEdgeConfigDtoMqtt> for TEdgeConfigReaderMqtt {
    fn from(value: TEdgeConfigDtoMqtt) -> Self {
        Self {
            port: value.port,
        }
    }
}

/// A list of all the keys that can be read with `tedge config get`
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ReadableKey {
    DeviceId,
    DeviceTy,
    // TODO amend this to store the optional key for the multi-value field
    C8yUrl,
    MqttPort,
}

impl fmt::Display for ReadableKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DeviceId => write!(f, "device.id"),
            Self::DeviceTy => write!(f, "device.type"),
            Self::C8yUrl => write!(f, "c8y.url"),
            Self::MqttPort => write!(f, "mqtt.port"),
        }
    }
}

impl FromStr for ReadableKey {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "device.id" => Ok(Self::DeviceId),
            "device.type" => Ok(Self::DeviceTy), 
            "c8y.url" => Ok(Self::C8yUrl),
            "mqtt.port" => Ok(Self::MqttPort),
            other => bail!("unrecognised key: {other}")
        }
    }
}

/// A list of all the keys that can be written to with `tedge config get`
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WritableKey {
    DeviceTy,
    // TODO amend this to store the optional key for the multi-value field
    C8yUrl,
    MqttPort,
}

impl fmt::Display for WritableKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DeviceTy => write!(f, "device.type"),
            Self::C8yUrl => write!(f, "c8y.url"),
            Self::MqttPort => write!(f, "mqtt.port"),
        }
    }
}

impl FromStr for WritableKey {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            key @ "device.id" => bail!("{key} is read-only"),
            "device.type" => Ok(Self::DeviceTy), 
            "c8y.url" => Ok(Self::C8yUrl),
            "mqtt.port" => Ok(Self::MqttPort),
            other => bail!("unrecognised key: {other}")
        }
    }
}

impl TEdgeConfigDto {
    pub fn try_update_str(&mut self, key: &WritableKey, value: &str) -> anyhow::Result<()> {
        match key {
            WritableKey::C8yUrl => self.c8y.url = value.to_owned(),
            WritableKey::DeviceTy => self.device.ty = value.to_owned(),
            WritableKey::MqttPort => self.mqtt.port = value.parse()?,
        }
        Ok(())
    }

}

impl TEdgeConfigReader {
    // This will be fallible once multi fields are introduced as the key might not exist in the config
    pub fn try_read_str<'a>(&'a self, key: &ReadableKey) -> anyhow::Result<Cow<'a, str>> {
        Ok(match key {
            ReadableKey::C8yUrl => (&self.c8y.url).into(),
            ReadableKey::DeviceId => (&self.device.id).into(),
            ReadableKey::DeviceTy => (&self.device.ty).into(),
            ReadableKey::MqttPort => self.mqtt.port.to_string().into(),
        })
    }
}

impl Default for TEdgeConfigDtoDevice {
    fn default() -> Self {
        Self {
            ty: "thin-edge.io".into(),
        }
    }
}

impl Default for TEdgeConfigDtoMqtt {
    fn default() -> Self {
        Self {
            port: 1883,
        }
    }
}


#[cfg(test)]
mod tests {
    use anyhow::Context;

    use super::*;

    #[test]
    fn parse_readable_key() {
        assert_eq!("device.type".parse::<ReadableKey>().unwrap(), ReadableKey::DeviceTy);    
        assert_eq!("c8y.url".parse::<ReadableKey>().unwrap(), ReadableKey::C8yUrl);    
        assert_eq!("c8y.cloud.url".parse::<ReadableKey>().unwrap(), todo!("Suitable ReadableKey value"));    
    }

    #[test]
    fn parse_writable_key() {
        assert_eq!("device.type".parse::<WritableKey>().unwrap(), WritableKey::DeviceTy);    
        assert_eq!("c8y.url".parse::<WritableKey>().unwrap(), WritableKey::C8yUrl);    
        assert_eq!("c8y.cloud.url".parse::<WritableKey>().unwrap(), todo!("Suitable WritableKey value"));    
    }

    fn read<'a>(rdr: &'a TEdgeConfigReader, key: &'static str) -> Cow<'a, str>{
        let key = key.parse().unwrap();
        rdr.try_read_str(&key).with_context(|| format!("reading {key:?} from config")).unwrap()
    }

    #[test]
    fn parse_dto_simple() {
        let toml = "c8y.url = \"https://c8y.example.com\"";
        let dto: TEdgeConfigDto= toml::from_str(&toml).unwrap();
        let rdr = TEdgeConfigReader::from(dto);
        assert_eq!(read(&rdr, "c8y.url"), "https://c8y.example.com");
        assert_eq!(read(&rdr, "device.type"), "thin-edge.io");
    }

    #[test]
    fn parse_dto_multi() {
        let toml = "c8y.cloud.url = \"https://c8y.example.com\"";
        let dto: TEdgeConfigDto= toml::from_str(&toml).unwrap();
        let rdr = TEdgeConfigReader::from(dto);
        assert_eq!(rdr.try_read_str(&ReadableKey::C8yUrl).unwrap(), "https://c8y.example.com");
        todo!("uncomment the assertion below when the code compiles")
        // assert_eq!(rdr.try_read_str(&ReadableKey::C8yUrl(Some("cloud".into()))).unwrap(), "https://c8y.example.com");
    }

    #[test]
    fn update_dto_simple() {
        let toml = "c8y.url = \"https://c8y.example.com\"";
        let mut dto: TEdgeConfigDto= toml::from_str(&toml).unwrap();

        let key = "c8y.url".parse().unwrap();
        dto.try_update_str(&key, "https://new.example.com").unwrap();
        let rdr = TEdgeConfigReader::from(dto);
        assert_eq!(read(&rdr, "c8y.url"), "https://new.example.com");
    }

    #[test]
    fn update_dto_multi() {
        let toml = "c8y.cloud.url = \"https://c8y.example.com\"";
        let mut dto: TEdgeConfigDto= toml::from_str(&toml).unwrap();

        let key = "c8y.cloud.url".parse().unwrap();
        dto.try_update_str(&key, "https://new.example.com").unwrap();
        let rdr = TEdgeConfigReader::from(dto);
        assert_eq!(read(&rdr, "c8y.cloud.url"), "https://new.example.com");
    }
}
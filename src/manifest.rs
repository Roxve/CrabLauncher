use serde::{de::Visitor, Deserialize};

#[derive(Debug)]
pub enum VersionKind {
    Release,
    Snapshot,
    OldAlpha,
    OldBeta,
}

impl<'de> Deserialize<'de> for VersionKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct VersionVistor;
        impl<'de> Visitor<'de> for VersionVistor {
            type Value = VersionKind;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "old_alpha, old_beta, release, or snapshot")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match v {
                    "old_alpha" => Ok(VersionKind::OldAlpha),
                    "old_beta" => Ok(VersionKind::OldBeta),
                    "release" => Ok(VersionKind::Release),
                    "snapshot" => Ok(VersionKind::Snapshot),
                    _ => Err(E::custom(format!("invaild value for VersionKind {}", v))),
                }
            }
        }

        deserializer.deserialize_str(VersionVistor)
    }
}

#[derive(Deserialize, Debug)]
pub struct Version {
    pub id: String,
    #[serde(rename = "type")]
    pub kind: VersionKind,
    pub url: String,
    #[allow(unused)]
    time: String,
    #[allow(unused)]
    #[serde(rename = "releaseTime")]
    release_time: String,
}

#[derive(Debug, Deserialize)]
pub struct Latest {
    pub release: String,
    pub snapshot: String,
}

#[derive(Debug, Deserialize)]
pub struct Manifest {
    pub latest: Latest,
    pub versions: Vec<Version>,
}

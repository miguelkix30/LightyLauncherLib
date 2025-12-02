use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[derive(Clone)]
pub struct FabricMetaData {
    pub arguments: FabricArguments,
    pub id: String,
    pub inheritsFrom: String,
    pub libraries: Vec<FabricLibrary>,
    pub mainClass: String,
    pub releaseTime: String,
    pub time: String,
    #[serde(rename = "type")]
    pub release_type: String,
}

#[derive(Debug, Deserialize)]
#[derive(Clone)]
pub struct FabricArguments {
    pub game: Vec<String>,
    pub jvm: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[derive(Clone)]
pub struct FabricLibrary {
    pub name: String,
    pub url: Option<String>,
    pub md5: Option<String>,
    pub sha1: Option<String>,
    pub sha256: Option<String>,
    pub sha512: Option<String>,
    pub size: Option<u64>,
}

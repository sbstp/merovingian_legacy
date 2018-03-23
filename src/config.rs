#[derive(Deserialize, Serialize)]
pub struct Config {
    pub movies_path: Option<PathBuf>,
    pub tv_path: Option<PathBuf>,
}

impl Config {}

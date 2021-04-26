#[derive(Debug)]
pub struct Addon {
  pub name: String,
  pub tmpdir: Result<tempfile::TempDir, std::io::Error>,
  pub extension: String,
  pub download_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ReleaseInfo {
  pub assets: Vec<Asset>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Asset {
  pub browser_download_url: String,
}

pub async fn download_addon(addon: Addon) -> Result<(), Box<dyn std::error::Error>> {
  let res = reqwest::Client::new()
    .get(addon.download_url)
    .header("Accept", "application/vnd.github.v3+json")
    .header("User-Agent", "gwlazy/0.1")
    .send()
    .await?;

  let git_info = res.json::<Vec<ReleaseInfo>>().await?;
  println!("{:#?}", git_info[0].assets[0].browser_download_url);
  Ok(())
}

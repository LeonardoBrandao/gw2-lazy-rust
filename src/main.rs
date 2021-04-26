mod utils;
use std::env;
use std::path::Path;
use tempfile;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let _gwpath = Path::new(&args[1]);
    let addons_arg = args[2].split_whitespace();
    let mut addons = Vec::new();

    for name in addons_arg {
        let ext;
        let download_url;
        match name {
            "arcdps" => ext = ".dll",
            _ => ext = ".zip",
        }
        match name {
            "arcdps" => download_url = "https://www.deltaconnected.com/arcdps/x64/d3d9.dll",
            "d912pxy" => download_url = "https://api.github.com/repos/megai2/d912pxy/releases",
            "gwradial" => {
                download_url = "https://api.github.com/repos/Friendly0Fire/GW2Radial/releases"
            }
            _ => download_url = "",
        }
        let addon = utils::Addon {
            name: name.to_string(),
            tmpdir: tempfile::TempDir::new(),
            extension: ext.to_string(),
            download_url: download_url.to_string(),
        };
        addons.push(addon);
    }

    for addon in addons {
        println!("Downloading -> {}", addon.name);
        let _r = utils::download_addon(addon).await;   
        println!("Done");
    }
    Ok(())
}

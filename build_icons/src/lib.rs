use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::{env, io};

use gvdb::gresource::{GResourceBuilder, GResourceFileData, PreprocessOptions};

pub const CONFIG_FILE: &str = "icons.toml";
const GENERAL_PREFIX: &str = "/org/gtkrs/icons/scalable/actions/";

const TARGET_FILE: &str = "resources.gresource";
const CONSTANTS_FILE: &str = "icon_modules.rs";

#[derive(Default, serde::Deserialize)]
pub struct Config {
    app_id: Option<String>,
    base_resource_path: Option<String>,
    icons_folder: Option<String>,
    icons: Option<Vec<String>>,
}

impl Config {
    fn load(dir: &str) -> Result<Self, io::Error> {
        let config_path: PathBuf = [dir, CONFIG_FILE].iter().collect();
        let config_file = std::fs::read_to_string(config_path)?;
        let mut config: Config =
            toml::from_str(&config_file).expect("Couldn't parse icon config file");

        if config.icons_folder.is_none() {
            let shipped_icons_path = option_env!("RELM4_ICONS_FOLDER");
            config.icons_folder = shipped_icons_path.map(|s| s.to_owned());
        }

        Ok(config)
    }
}

pub fn path_to_icon_name(string: &OsStr) -> String {
    match string.to_str() {
        Some(string) => {
            if string.ends_with(".svg") {
                string
                    .trim_end_matches("-symbolic.svg")
                    .trim_end_matches(".svg")
                    .to_owned()
            } else {
                panic!("Found non-icon file `{string}`");
            }
        }
        None => panic!("Failed to convert file name `{string:?}` to string"),
    }
}

pub fn build_icons(manifest_path: &str) {
    let out_dir = env::var("OUT_DIR").unwrap();
    let manifest_dir = Path::new(&manifest_path).canonicalize().unwrap();
    eprintln!("Canonical manifest dir: {manifest_dir:?}");

    let (config, config_dir) = if cfg!(docsrs) {
        if let Ok(source_dir) = env::var("SOURCE_DIR") {
            (Config::load(&source_dir).unwrap_or_default(), source_dir)
        } else {
            (Config::default(), "".into())
        }
    } else {
        (
            Config::load(manifest_path).expect("couldn't load manifest"),
            manifest_dir.to_str().unwrap().to_owned(),
        )
    };

    eprintln!("Canonical config dir: {config_dir:?}");
    println!("cargo:rerun-if-changed={config_dir}/icons.toml");

    let mut icons: HashMap<String, PathBuf> = HashMap::new();

    if let Some(folder) = &config.icons_folder {
        println!("cargo:rerun-if-changed={folder}");
        let custom_icons_path: PathBuf = [&config_dir, folder].iter().collect();
        let read_dir = std::fs::read_dir(custom_icons_path)
            .expect("Couldn't open icon path specified in config (relative to the manifest)");
        for entry in read_dir {
            let entry = entry.unwrap();
            let icon_modules = path_to_icon_name(&entry.file_name());
            if icons.insert(icon_modules.clone(), entry.path()).is_some() {
                panic!("Icon with name `{icon_modules}` exists twice")
            }
        }
    }

    println!(
        "RELM4_ICONS_FOLDER={}",
        std::env::var("RELM4_ICONS_FOLDER").unwrap_or_default()
    );
    let icons_folder = config
        .icons_folder
        .expect("Could not find icons folder specified in config");

    if let Some(icon_modules) = config.icons {
        let dirs = std::fs::read_dir(icons_folder).expect("Couldn't open folder of shipped icons");
        let dirs: Vec<_> = dirs
            .map(|entry| {
                let entry = entry.expect("Couldn't open directories in shipped icon folder");
                entry.path()
            })
            .collect();

        'outer: for icon_modules in icon_modules {
            for dir in &dirs {
                let icon_file_name = format!("{icon_modules}-symbolic.svg");
                let icon_path = dir.join(icon_file_name);
                if icon_path.exists() {
                    if icons.insert(icon_modules.clone(), icon_path).is_some() {
                        panic!("Icon with name `{icon_modules}` exists twice")
                    }
                    continue 'outer;
                }
            }
            panic!("Icon {icon_modules} not found in shipped icons");
        }
    }

    let prefix = if let Some(base_resource_path) = &config.base_resource_path {
        format!("{}icons/scalable/actions/", base_resource_path)
    } else if let Some(app_id) = &config.app_id {
        format!("/{}/icons/scalable/actions/", app_id.replace('.', "/"))
    } else {
        GENERAL_PREFIX.into()
    };

    // Generate resource bundle
    let resources = icons
        .iter()
        .map(|(icon, path)| {
            GResourceFileData::from_file(
                [&prefix, icon, "-symbolic.svg"].into_iter().collect(),
                path,
                true,
                &PreprocessOptions::xml_stripblanks(),
            )
            .unwrap()
        })
        .collect();

    let data = GResourceBuilder::from_file_data(resources)
        .build()
        .expect("Failed to build resource bundle");

    std::fs::write(Path::new(&out_dir).join(TARGET_FILE), data).unwrap();

    // Create file that contains the icon names as constants
    let constants: String = icons
        .iter()
        .map(|(icon_modules, icon_path)| {
            let const_name = icon_modules.to_uppercase().replace('-', "_");
            format!(
                "
            /// Icon name of the icon `{icon_modules}`, found at `{icon_path:?}`.
            pub const {const_name}: &str = \"{icon_modules}\";
            "
            )
        })
        .chain([format!(
            "pub(crate) const APP_ID: &str = \"{}\";",
            config.app_id.unwrap_or_default()
        )])
        .chain([format!(
            "pub(crate) const BASE_RESOURCE_PATH: &str = \"{}\";",
            config.base_resource_path.unwrap_or_default()
        )])
        .collect();

    std::fs::write(Path::new(&out_dir).join(CONSTANTS_FILE), constants).unwrap();
}

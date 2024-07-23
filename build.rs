use std::{
    collections::HashMap,
    env,
    ffi::OsStr,
    io,
    path::{Path, PathBuf},
};

fn main() {
    // let out_dir = env::var("OUT_DIR").unwrap();
    // let mut manifest_dir = relm4_icons_build::manifest_path::SHIPPED_ICONS_PATH.to_owned();

    // eprintln!("Canonical manifest dir: {manifest_dir:?}");

    // let (config, config_dir) = if cfg!(docsrs) {
    //     if let Ok(source_dir) = env::var("SOURCE_DIR") {
    //         (Config::load(&source_dir).unwrap_or_default(), source_dir)
    //     } else {
    //         (Config::default(), "".into())
    //     }
    // } else {
    //     // Try finding the target directory which is just below the manifest directory
    //     // of the user.
    //     // Unfortunately, the CARGO_MANIFEST_DIR env var passed by cargo always points
    //     // to this crate, so we wouldn't find the users config file this way.
    //     while !manifest_dir.join("Cargo.toml").exists() {
    //         if !manifest_dir.pop() {
    //             panic!("Couldn't find your manifest directory");
    //         }
    //     }
    //     let config_dir = manifest_dir
    //         .to_str()
    //         .expect("Couldn't convert manifest directory to string")
    //         .to_owned();
    //     (
    //         Config::load(&config_dir).expect("Couldn't find `icons.toml` next to `Cargo.toml`"),
    //         config_dir,
    //     )
    // };

    // eprintln!("Canonical config dir: {config_dir:?}");
    // println!("cargo:rerun-if-changed={config_dir}/icons.toml");

    // let mut icons: HashMap<String, PathBuf> = HashMap::new();

    // if let Some(folder) = &config.icon_folder {
    //     println!("cargo:rerun-if-changed={folder}");
    //     let custom_icons_path: PathBuf = [&config_dir, folder].iter().collect();
    //     let read_dir = std::fs::read_dir(custom_icons_path)
    //         .expect("Couldn't open icon path specified in config (relative to the manifest)");
    //     for entry in read_dir {
    //         let entry = entry.unwrap();
    //         let icon_name = path_to_icon_name(&entry.file_name());
    //         if icons.insert(icon_name.clone(), entry.path()).is_some() {
    //             panic!("Icon with name `{icon_name}` exists twice")
    //         }
    //     }
    // }

    // if let Some(icon_names) = config.icons {
    //     let dirs =
    //         std::fs::read_dir(SHIPPED_ICONS_PATH).expect("Couldn't open folder of shipped icons");
    //     let dirs: Vec<_> = dirs
    //         .map(|entry| {
    //             let entry = entry.expect("Couldn't open directories in shipped icon folder");
    //             entry.path()
    //         })
    //         .collect();

    //     'outer: for icon_name in icon_names {
    //         for dir in &dirs {
    //             let icon_file_name = format!("{icon_name}-symbolic.svg");
    //             let icon_path = dir.join(icon_file_name);
    //             if icon_path.exists() {
    //                 if icons.insert(icon_name.clone(), icon_path).is_some() {
    //                     panic!("Icon with name `{icon_name}` exists twice")
    //                 }
    //                 continue 'outer;
    //             }
    //         }
    //         panic!("Icon {icon_name} not found in shipped icons");
    //     }
    // }

    // let prefix = if let Some(base_resource_path) = &config.base_resource_path {
    //     format!("{}icons/scalable/actions/", base_resource_path)
    // } else if let Some(app_id) = &config.app_id {
    //     format!("/{}/icons/scalable/actions/", app_id.replace('.', "/"))
    // } else {
    //     GENERAL_PREFIX.into()
    // };

    // // Generate resource bundle
    // let resources = icons
    //     .iter()
    //     .map(|(icon, path)| {
    //         GResourceFileData::from_file(
    //             [&prefix, icon, "-symbolic.svg"].into_iter().collect(),
    //             path,
    //             true,
    //             &PreprocessOptions::xml_stripblanks(),
    //         )
    //         .unwrap()
    //     })
    //     .collect();

    // let data = GResourceBuilder::from_file_data(resources)
    //     .build()
    //     .expect("Failed to build resource bundle");

    // std::fs::write(Path::new(&out_dir).join(TARGET_FILE), data).unwrap();

    // // Create file that contains the icon names as constants
    // let constants: String = icons
    //     .iter()
    //     .map(|(icon_name, icon_path)| {
    //         let const_name = icon_name.to_uppercase().replace('-', "_");
    //         format!(
    //             "
    //         /// Icon name of the icon `{icon_name}`, found at `{icon_path:?}`.
    //         pub const {const_name}: &str = \"{icon_name}\";
    //         "
    //         )
    //     })
    //     .chain([format!(
    //         "pub(crate) const APP_ID: &str = \"{}\";",
    //         config.app_id.unwrap_or_default()
    //     )])
    //     .chain([format!(
    //         "pub(crate) const BASE_RESOURCE_PATH: &str = \"{}\";",
    //         config.base_resource_path.unwrap_or_default()
    //     )])
    //     .collect();

    // std::fs::write(Path::new(&out_dir).join(CONSTANTS_FILE), constants).unwrap();
}

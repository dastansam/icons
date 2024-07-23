use std::{
    env,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

const SHIPPED_ICONS_PATH: &str = "icons";
const ICON_MODULE_FILE: &str = "icon_modules.rs";

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join(ICON_MODULE_FILE);
    let mut icon_modules = File::create(dest_path).unwrap();

    let dirs = fs::read_dir(SHIPPED_ICONS_PATH).expect("Couldn't open folder of shipped icons");

    // let dirs: Vec<_> = dirs
    //     .map(|entry| {
    //         let entry = entry.expect("Couldn't open directories in shipped icon folder");
    //         entry.path()
    //     })
    //     .collect();

    for entry in dirs.flatten() {
        eprintln!("cargo:rerun-if-changed={}", entry.path().display());
        let path = entry.path();

        // loop through all folders for icons
        for entry in fs::read_dir(&path).expect("Couldn't open folder of shipped icons") {
            let entry = entry.expect("Couldn't open directories in shipped icon folder");
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "svg") {
                let icon_name = path.file_stem().unwrap().to_str().unwrap();

                // replace dash with underscore
                let module_icon_name = icon_name.replace("-", "_");
                println!("Writing icon module for {}", module_icon_name);
                writeln!(
                    icon_modules,
                    "pub mod {} {{
                        use crate::LazyIcon;
                        pub static ICON: LazyIcon = LazyIcon {{
                            name: \"{}\",
                            data: include_bytes!(\"{}\"),
                            initialized: std::sync::OnceLock::new(),
                        }};
                    }}",
                    module_icon_name,
                    icon_name,
                    path.canonicalize().unwrap().to_str().unwrap()
                )
                .unwrap();
            }
        }
    }
}

use flapigen::{JavaConfig, LanguageConfig};
use std::{env, fs, path::Path};

fn main() {
    env_logger::init();
    let out_dir = env::var("OUT_DIR").unwrap();
    let in_src = Path::new("src").join("java_glue.rs.in");
    let out_src = Path::new(&out_dir).join("java_glue.rs");
    let java_folder = Path::new("../app/src/main/java/com/nixalpha/rustapplication/lib");
    if java_folder.exists() {
        fs::remove_dir_all(java_folder);
    }
    fs::create_dir(java_folder).unwrap();
    //ANCHOR: config
    let swig_gen = flapigen::Generator::new(LanguageConfig::JavaConfig(
        JavaConfig::new(
            java_folder.into(),
            "com.nixalpha.rustapplication.lib".into(),
        )
        .use_null_annotation_from_package("androidx.annotation".into()),
    ))
    .rustfmt_bindings(true);
    //ANCHOR_END: config
    swig_gen.expand("android bindings", &in_src, &out_src);
    println!("cargo:rerun-if-changed={}", in_src.display());
}

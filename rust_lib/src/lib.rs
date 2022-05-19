use log::info;
mod java_glue;

use std::fs;

pub use crate::java_glue::*;

// ANCHOR: rust_code
struct Session {
    a: i32,
}

impl Session {
    pub fn new(a_num: i32) -> Session {
        #[cfg(target_os = "android")]
        android_logger::init_once(
            android_logger::Config::default()
                .with_min_level(log::Level::Debug)
                .with_tag("Hello"),
        );
        log_panics::init(); // log panics rather than printing them
        info!("init log system - done");
        Session { a: a_num }
    }

    pub fn add_and1(&self, val: i32) -> i32 {
        self.a + val + 1
    }

    // Greeting with full, no-runtime-cost support for newlines and UTF-8
    pub fn greet(to: &str) -> String {
        format!("Hello {} 👋\nIt's a pleasure to meet you!", to)
    }

    pub fn lspath(dir: &str) -> String {
        let paths = fs::read_dir(dir).unwrap();
        paths
            .map(|p| p.unwrap().path().display().to_string())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

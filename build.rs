extern crate cc;

use std::env;

fn main() {
    println!("{}", env::consts::OS);

    let mut builder = cc::Build::new();

    builder.files(vec!["src/tray.c"]);

    #[cfg(target_os = "windows")]
    {
        builder.define("TRAY_WINAPI", Some("1"));
    }

    #[cfg(target_os = "linux")]
    {
        extern crate pkg_config;
        pkg_config::Config::new().probe("appindicator3");

        builder.define("TRAY_APPINDICATOR", Some("1"));
    }

    #[cfg(target_os = "macos")]
    {
        builder.define("TRAY_APPKIT", Some("1"));
    }

    builder.compile("tray");
}

use anyhow::Result;
use clap::{App, Arg};
use walkdir::WalkDir;
use zip::ZipArchive;

fn main() -> Result<()> {
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"));

    let matches = App::new("log4j-scan")
        .version(env!("CARGO_PKG_VERSION"))
        .setting(clap::AppSettings::ArgRequiredElseHelp)
        .arg(Arg::with_name("PATH").required(true).help("path to scan"))
        .get_matches();

    log::info!("Walking {}", matches.value_of("PATH").unwrap());

    for entry in WalkDir::new(matches.value_of("PATH").unwrap())
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        log::debug!("Walking {}", entry.path().display());
        if entry.file_name() == "Logger.class" && entry.path().to_str().unwrap_or("").contains("log4j") {
            println!("extracted log4j found in {}", entry.path().display());
        } else if entry.file_name().to_str().unwrap_or("").ends_with(".jar") {
            if let Ok(reader) = std::fs::File::open(entry.path()) {
                if let Ok(mut zip) = ZipArchive::new(reader) {
                    for i in 0..zip.len() {
                        if let Ok(file) = zip.by_index(i) {
                            let name = file.name();
                            if name.ends_with("Logger.class") && name.contains("log4j") {
                                println!("packed log4j found in {}", entry.path().display());
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

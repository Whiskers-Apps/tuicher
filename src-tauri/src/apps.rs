use std::{error::Error, fs, path::PathBuf, sync::mpsc::channel, thread};

use bincode::{config, Decode, Encode};
use freedesktop_desktop_entry::{default_paths, get_languages_from_env, DesktopEntry, Iter};
use notify::{Event, Watcher};
use serde::{Deserialize, Serialize};
use tux_icons::icon_fetcher::IconFetcher;

#[derive(Serialize, Deserialize, Encode, Decode, Debug, Clone)]
pub struct App {
    pub name: String,
    pub description: Option<String>,
    pub keywords: Vec<String>,
    pub path: PathBuf,
    pub icon_path: Option<PathBuf>,
}

pub fn setup_apps_indexing() -> Result<(), Box<dyn Error>> {
    thread::spawn(move || {
        let _ = index_apps();
    });

    thread::spawn(|| {
        let (tx, rx) = channel::<notify::Result<Event>>();
        let mut watcher = notify::recommended_watcher(tx).expect("Failed to get watcher");

        for path in default_paths() {
            if !path.exists() {
                continue;
            }

            watcher
                .watch(&path, notify::RecursiveMode::Recursive)
                .expect("Failed to add path to watcher");
        }

        for res in rx {
            match res {
                Ok(event) => match event.kind {
                    notify::EventKind::Create(_) => {
                        let _ = index_apps();
                    }
                    notify::EventKind::Modify(modify_kind) => match modify_kind {
                        notify::event::ModifyKind::Data(_data_change) => {
                            let _ = index_apps();
                        }
                        _ => {}
                    },
                    notify::EventKind::Remove(_) => {
                        let _ = index_apps();
                    }
                    _ => {}
                },
                Err(_) => {}
            }
        }
    });

    Ok(())
}

fn index_apps() -> Result<(), Box<dyn Error>> {
    let icon_fetcher = IconFetcher::new().set_return_target_path(true);

    let locales = get_languages_from_env();
    let entries = Iter::new(default_paths())
        .entries(Some(&locales))
        .collect::<Vec<_>>()
        .into_iter()
        .filter_map(|entry| {
            if entry.no_display() {
                return None;
            }

            if let Some(type_) = entry.type_() {
                if type_ == "Application" {
                    return Some(entry);
                }
            }

            None
        })
        .collect::<Vec<DesktopEntry>>();

    let mut apps: Vec<App> = vec![];

    for entry in entries {
        let name = match entry.name(&locales) {
            Some(name) => name.to_string(),
            None => continue,
        };

        let description = match entry.comment(&locales) {
            Some(description) => Some(description.to_string()),
            None => None,
        };

        let keywords: Vec<String> = match entry.keywords(&locales) {
            Some(keywords) => keywords.into_iter().map(|key| key.to_string()).collect(),
            None => vec![],
        };

        let icon_path = if let Some(icon) = entry.icon() {
            icon_fetcher.get_icon_path(icon)
        } else {
            None
        };

        apps.push(App {
            name,
            description,
            keywords,
            path: entry.path,
            icon_path: icon_path,
        });
    }

    let bytes = bincode::encode_to_vec(&apps, config::standard())?;

    fs::write(&get_apps_indexing_path()?, &bytes)?;

    Ok(())
}

fn get_apps_indexing_path() -> Result<PathBuf, Box<dyn Error>> {
    let mut path = dirs::cache_dir()
        .ok_or_else(|| "Failed to get cache dir")?
        .join("tuicher");

    if !path.exists() {
        fs::create_dir_all(&path)?;
    }

    path.push("apps.bin");

    Ok(path)
}

pub fn get_apps() -> Result<Vec<App>, Box<dyn Error>> {
    let bytes = fs::read(get_apps_indexing_path()?)?;
    let (apps, _): (Vec<App>, usize) = bincode::decode_from_slice(&bytes, config::standard())?;

    Ok(apps)
}

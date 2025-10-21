use std::error::Error;

use sniffer_rs::sniffer::Sniffer;
use tauri::Window;
use tuicher_rs::{
    config::{get_config, write_config, BookmarkConfig, Config},
    result::{Action, AddBookmark, Bookmark, OpenURL, RemoveBookmark, TUIResult},
    utils::Query,
};

pub fn get_bookmarks_results(
    config: &Config,
    sniffer: &Sniffer,
    search_text: &str,
) -> Result<Vec<TUIResult>, Box<dyn Error>> {
    let query = Query::new(search_text)?;
    let search_text = search_text.to_string();

    if let Some(keyword) = &query.keyword {
        if keyword == "a" || keyword == "add" {
            let parts: Vec<&str> = search_text.split(" ").collect();

            let name = if parts.len() >= 3 {
                parts.clone()[1..parts.len() - 1].join(" ")
            } else if parts.len() >= 2 {
                parts.clone().get(1).unwrap().to_string()
            } else {
                "".to_string()
            };

            let url = if parts.len() >= 3 {
                parts
                    .clone()
                    .get(parts.len() - 1)
                    .ok_or_else(|| "Error getting url")?
                    .to_string()
            } else {
                "".to_string()
            };

            return Ok(vec![TUIResult::new(
                format!("Add: {name}"),
                "bookmarks".to_string(),
            )
            .set_secondary_text(format!("URL: {url}"))
            .set_action(Action::Bookmark(Bookmark::Add(AddBookmark {
                name: name.clone(),
                url: url.clone(),
            })))]);
        }

        if keyword == "r" || keyword == "remove" {
            let results: Vec<TUIResult> =
                config
                    .bookmarks
                    .clone()
                    .into_iter()
                    .filter_map(|bookmark| {
                        if sniffer.matches(&bookmark.name.clone(), &query.get_query()) {
                            Some(
                                TUIResult::new(
                                    format!("Remove {}", &bookmark.name),
                                    "bookmarks".to_string(),
                                )
                                .set_secondary_text(&bookmark.url)
                                .set_action(Action::Bookmark(Bookmark::Remove(RemoveBookmark {
                                    id: bookmark.id.clone(),
                                }))),
                            )
                        } else {
                            None
                        }
                    })
                    .collect();

            return Ok(results);
        }
    }

    let results: Vec<TUIResult> = config
        .bookmarks
        .iter()
        .filter_map(|bookmark| {
            if sniffer.matches(&bookmark.name, &search_text) {
                Some(
                    TUIResult::new(&bookmark.name, &"bookmarks".to_string())
                        .set_secondary_text(&bookmark.url)
                        .set_action(Action::OpenURL(OpenURL::new(&bookmark.url))),
                )
            } else {
                None
            }
        })
        .collect();

    Ok(results)
}

pub fn on_bookmark_action(bookmark: Bookmark, window: Window) -> Result<(), Box<dyn Error>> {
    match bookmark {
        Bookmark::Add(add_bookmark) => {
            let mut config = get_config()?;
            let last_id = config
                .bookmarks
                .iter()
                .map(|conf| conf.id)
                .max()
                .unwrap_or(0);

            let new_bookmark = BookmarkConfig {
                id: last_id + 1,
                name: add_bookmark.name.clone(),
                url: add_bookmark.url.clone(),
            };

            config.bookmarks.push(new_bookmark);

            write_config(&config)?;
        }
        Bookmark::Remove(remove_bookmark) => {
            let mut config = get_config()?;

            config.bookmarks = config
                .bookmarks
                .into_iter()
                .filter(|bookmark| bookmark.id != remove_bookmark.id)
                .collect();

            write_config(&config)?;
        }
    }

    window.close()?;
    Ok(())
}

use sniffer_rs::sniffer::Sniffer;
use tuicher_rs::{
    config::{get_config, SearchEngine},
    result::{Action, OpenApp, OpenURL, TUIResult},
    utils::Query,
};

use crate::{
    apps::get_apps,
    plugins::{
        bookmarks::get_bookmarks_results, emojis::get_emoji_results, session::get_session_results,
    },
};

#[tauri::command]
pub fn invoke_search(text: String) -> Result<Vec<TUIResult>, String> {
    if text.is_empty() {
        return Ok(vec![]);
    }

    let config = get_config().map_err(|_| "Failed to get config")?;
    let sniffer = Sniffer::new();
    let search_query = Query::new(&text).map_err(|e| e.to_string())?;
    let mut results: Vec<TUIResult> = vec![];

    if sniffer.matches("Settings", &search_query.full_text) {
        results.push(
            TUIResult::new("Settings", "settings")
                .set_secondary_text("Open Tuicher Settings")
                .set_action(Action::OpenSettings),
        );
    };

    if let Some(keyword) = search_query.keyword.clone() {
        let search_engine = config
            .search_engines
            .clone()
            .into_iter()
            .find(|engine| &engine.keyword == &keyword);

        if let Some(search_engine) = search_engine {
            return Ok(vec![get_search_engine_result(
                &search_engine,
                &search_query.get_query().clone(),
            )]);
        }

        if config.enable_session_manager && keyword == config.session_manager_keyword {
            return Ok(get_session_results(&sniffer, &search_query.get_query())
                .map_err(|e| e.to_string())?);
        }

        if config.enable_emojis && keyword == config.emojis_keyword {
            return Ok(get_emoji_results(&sniffer, &search_query.get_query()));
        }

        if config.enable_bookmarks && keyword == config.bookmarks_keyword {
            return Ok(
                get_bookmarks_results(&config, &sniffer, &search_query.get_query())
                    .map_err(|e| e.to_string())?,
            );
        }
    }

    let mut apps: Vec<TUIResult> = get_apps()
        .unwrap_or(vec![])
        .into_iter()
        .filter_map(|app| {
            let matches_keywords = app
                .clone()
                .keywords
                .iter()
                .any(|key| sniffer.matches(key, &text));

            if sniffer.matches(&app.name, &text) || matches_keywords {
                let action = Action::OpenApp(OpenApp::new(&app.path));

                let mut result = TUIResult::new(&app.name, &"app".to_string())
                    .set_secondary_text(if let Some(description) = app.description {
                        description
                    } else {
                        "Application".to_string()
                    })
                    .set_action(action);

                if let Some(icon_path) = app.icon_path {
                    result.set_icon_path(icon_path);
                }

                return Some(result);
            }

            None
        })
        .collect();

    if !apps.is_empty() {
        results.append(&mut apps);
        return Ok(results);
    }

    let default_search_engine = config
        .clone()
        .search_engines
        .into_iter()
        .find(|search_engine| search_engine.id == config.default_search_engine);

    if let Some(search_engine) = default_search_engine {
        return Ok(vec![get_search_engine_result(&search_engine, &text)]);
    }

    Ok(vec![])
}

fn get_search_engine_result(search_engine: &SearchEngine, text: &str) -> TUIResult {
    let search_query = search_engine.url.clone().replace("%s", text);
    let action = Action::OpenURL(OpenURL::new(search_query));

    TUIResult::new(&search_engine.name, &"search-engine".to_string())
        .set_secondary_text(format!("Search for {}", &text))
        .set_action(action)
}

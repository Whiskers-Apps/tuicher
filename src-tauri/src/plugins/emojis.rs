use sniffer_rs::sniffer::Sniffer;
use tuicher_rs::result::{Action, CopyText, TUIResult};

pub fn get_emoji_results(sniffer: &Sniffer, search_text: &str) -> Vec<TUIResult> {
    if search_text.trim().is_empty() {
        return vec![];
    }

    let results = emojis::iter()
        .into_iter()
        .filter_map(|emoji| {
            let matches_codes = emoji
                .shortcodes()
                .into_iter()
                .any(|code| sniffer.matches(code, search_text));

            if sniffer.matches(emoji.name(), search_text) || matches_codes {
                return Some(
                    TUIResult::new(
                        format!("{} {}", emoji.as_str(), emoji.name()),
                        "emojis".to_string(),
                    )
                    .set_action(Action::CopyText(CopyText::new(emoji.as_str()))),
                );
            }

            None
        })
        .collect();

    results
}

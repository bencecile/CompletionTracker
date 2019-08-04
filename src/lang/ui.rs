use serde_derive::{Serialize};

use crate::lang::{UILangStrings};

/// These are all of the language strings for the front-end UI
#[derive(Serialize)]
pub struct UIStrings {
    // These are the UILang short strings
    pub en: UILangStrings,
    pub jp: UILangStrings,

    pub aliases: UILangStrings,
    pub aliases_new: UILangStrings,
    pub app_title: UILangStrings,
    pub cancel: UILangStrings,
    pub characters: UILangStrings,
    pub companies: UILangStrings,
    pub create: UILangStrings,
    pub description: UILangStrings,
    pub delete: UILangStrings,
    pub edit: UILangStrings,
    pub file_choose: UILangStrings,
    pub file_error: UILangStrings,
    pub file_not_selected: UILangStrings,
    pub filter: UILangStrings,
    pub github_footer: UILangStrings,
    pub home: UILangStrings,
    pub image: UILangStrings,
    pub language: UILangStrings,
    pub management: UILangStrings,
    pub missing_page: UILangStrings,
    pub my_completion: UILangStrings,
    pub name: UILangStrings,
    pub need_json_request: UILangStrings,
    pub people: UILangStrings,
    pub release_date: UILangStrings,
    pub related_link_new: UILangStrings,
    pub related_links: UILangStrings,
    pub running_version: UILangStrings,
    pub save: UILangStrings,
    pub search: UILangStrings,
    pub sending_request: UILangStrings,
    pub settings: UILangStrings,
    pub shut_down: UILangStrings,
    pub shutdown_response: UILangStrings,
    pub source_new: UILangStrings,
    pub sources: UILangStrings,
    pub sources_search_pl: UILangStrings,
    pub stats: UILangStrings,
    pub undo: UILangStrings,
    pub undo_error: UILangStrings,
    pub undo_success: UILangStrings,
    pub universe_tag: UILangStrings,
    pub universe_tag_create: UILangStrings,
    pub universe_tag_created: UILangStrings,
    pub universe_tag_created_view: UILangStrings,
    pub universe_tag_new: UILangStrings,
    pub universe_tags: UILangStrings,
    pub wishlist: UILangStrings,
}
impl UIStrings {
    pub fn new() -> UIStrings {
        UIStrings {
            en: UILangStrings {
                en: "English".to_string(),
                jp: "英語".to_string(),
            },
            jp: UILangStrings {
                en: "Japanese".to_string(),
                jp: "日本語".to_string(),
            },

            aliases: UILangStrings {
                en: "Aliases".to_string(),
                jp: "アダ名".to_string(),
            },
            aliases_new: UILangStrings {
                en: "Create a new alias".to_string(),
                jp: "新しいアダ名を作成して".to_string(),
            },
            app_title: UILangStrings {
                en: "Completion Tracker".to_string(),
                jp: "成績トラッカ".to_string(),
            },
            cancel: UILangStrings {
                en: "Cancel".to_string(),
                jp: "キャンセル".to_string(),
            },
            characters: UILangStrings {
                en: "Characters".to_string(),
                jp: "全てのキャラ".to_string(),
            },
            companies: UILangStrings {
                en: "Companies".to_string(),
                jp: "全ての会社".to_string(),
            },
            create: UILangStrings {
                en: "Create".to_string(),
                jp: "作成".to_string(),
            },
            description: UILangStrings {
                en: "Description".to_string(),
                jp: "詳細".to_string(),
            },
            delete: UILangStrings {
                en: "Delete".to_string(),
                jp: "削除".to_string(),
            },
            edit: UILangStrings {
                en: "Edit".to_string(),
                jp: "変更".to_string(),
            },
            file_choose: UILangStrings {
                en: "Choose a file".to_string(),
                jp: "ファイルを選択".to_string(),
            },
            file_error: UILangStrings {
                en: "An error occurred while reading in the file".to_string(),
                jp: "ファイルの読み込み中でエラーが発生しました".to_string(),
            },
            file_not_selected: UILangStrings {
                en: "No file is selected".to_string(),
                jp: "ファイルは選択していません".to_string(),
            },
            filter: UILangStrings {
                en: "Filter".to_string(),
                jp: "絞り込む".to_string(),
            },
            github_footer: UILangStrings {
                en: "View on GitHub".to_string(),
                jp: "GitHubでご覧".to_string(),
            },
            home: UILangStrings {
                en: "Home".to_string(),
                jp: "ホーム".to_string(),
            },
            image: UILangStrings {
                en: "Image".to_string(),
                jp: "画像".to_string(),
            },
            language: UILangStrings {
                en: "Language".to_string(),
                jp: "言語".to_string(),
            },
            management: UILangStrings {
                en: "Management".to_string(),
                jp: "管理".to_string(),
            },
            missing_page: UILangStrings {
                en: "Looks like this page is missing. Try going Home or somewhere else.".to_string(),
                jp: "このページは行方不明みたいです。ホームか別の場所へ行ってみましょう。".to_string(),
            },
            my_completion: UILangStrings {
                en: "My Completion".to_string(),
                jp: "私の成績".to_string(),
            },
            name: UILangStrings {
                en: "Name".to_string(),
                jp: "名".to_string(),
            },
            need_json_request: UILangStrings {
                en: "The POST body must be JSON".to_string(),
                jp: "POST本文はJSONに必要です".to_string(),
            },
            people: UILangStrings {
                en: "People".to_string(),
                jp: "人々".to_string(),
            },
            release_date: UILangStrings {
                en: "Release Date".to_string(),
                jp: "初発行日".to_string(),
            },
            related_link_new: UILangStrings {
                en: "Create a new related link".to_string(),
                jp: "新しい関連リンクを作成して".to_string(),
            },
            related_links: UILangStrings {
                en: "Related Links".to_string(),
                jp: "関連リンク".to_string(),
            },
            running_version: UILangStrings {
                en: "Running Version".to_string(),
                jp: "発動バージョン".to_string(),
            },
            save: UILangStrings {
                en: "Save".to_string(),
                jp: "セーブ".to_string(),
            },
            search: UILangStrings {
                en: "Search".to_string(),
                jp: "検索".to_string(),
            },
            sending_request: UILangStrings {
                en: "The request is being sent…".to_string(),
                jp: "リクエストが送りかけます…".to_string(),
            },
            settings: UILangStrings {
                en: "Settings".to_string(),
                jp: "設定".to_string(),
            },
            shut_down: UILangStrings {
                en: "Shut down".to_string(),
                jp: "シャットダウン".to_string(),
            },
            shutdown_response: UILangStrings {
                en: "Shutting down the server…".to_string(),
                jp: "サーバがシャットダウンしています…".to_string(),
            },
            source_new: UILangStrings {
                en: "New Source".to_string(),
                jp: "新しいソース".to_string(),
            },
            sources: UILangStrings {
                en: "Sources".to_string(),
                jp: "全てのソース".to_string(),
            },
            sources_search_pl: UILangStrings {
                en: "Search for Sources, Universes, Series, Arcs, etc.".to_string(),
                jp: "ソースや世界やシリーズやアークなどと検索してみよう".to_string(),
            },
            stats: UILangStrings {
                en: "Stats".to_string(),
                jp: "統計".to_string(),
            },
            undo: UILangStrings {
                en: "Undo".to_string(),
                jp: "取り消す".to_string(),
            },
            undo_error: UILangStrings {
                en: "The undo has failed".to_string(),
                jp: "取り消しが失敗しました".to_string(),
            },
            undo_success: UILangStrings {
                en: "The undo has completed successfully".to_string(),
                jp: "取り消しが成功しました".to_string(),
            },
            universe_tag: UILangStrings {
                en: "Universe Tag".to_string(),
                jp: "世界ターグ".to_string(),
            },
            universe_tag_create: UILangStrings {
                en: "Create a Universe Tag".to_string(),
                jp: "世界ターグを作成".to_string(),
            },
            universe_tag_created: UILangStrings {
                en: "A Universe Tag has been created".to_string(),
                jp: "世界ターグが作成されました".to_string(),
            },
            universe_tag_created_view: UILangStrings {
                en: "View the created Universe Tag".to_string(),
                jp: "作成した世界ターグに行って見ましょう".to_string(),
            },
            universe_tag_new: UILangStrings {
                en: "New Universe Tag".to_string(),
                jp: "新しい世界ターグ".to_string(),
            },
            universe_tags: UILangStrings {
                en: "Universe Tags".to_string(),
                jp: "全ての世界ターグ".to_string(),
            },
            wishlist: UILangStrings {
                en: "Wishlist".to_string(),
                jp: "ほしい物リスト".to_string(),
            },
        }
    }
}

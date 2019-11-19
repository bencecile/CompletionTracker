const LOCALE_EN = "en";
const LOCALE_JA = "jp";

// Having them closer together is nicer to see what we're using
const allLangStrings = {
    // -------- Translations for the content Langs --------
    English: {
        [LOCALE_EN]: "English",
        [LOCALE_JA]: "英語",
    },
    Japanese: {
        [LOCALE_EN]: "Japanese",
        [LOCALE_JA]: "日本語",
    },

    completion_tracker: {
        [LOCALE_EN]: "Completion Tracker",
        [LOCALE_JA]: "成績トラッカー",
    },
    current_content_lang: {
        [LOCALE_EN]: "Current Content Language",
        [LOCALE_JA]: "現在の内容言語",
    },
    description: {
        [LOCALE_EN]: "Description",
        [LOCALE_JA]: "説明",
    },
    error_occurred: {
        [LOCALE_EN]: "An error occured: {0}",
        [LOCALE_JA]: "エラーが発生しました：{0}",
    },
    home: {
        [LOCALE_EN]: "Home",
        [LOCALE_JA]: "ホーム",
    },
    loading: {
        [LOCALE_EN]: "Loading…",
        [LOCALE_JA]: "ロード中…",
    },
    name: {
        [LOCALE_EN]: "Name",
        [LOCALE_JA]: "名",
    },

    error_bad_route_params: {
        [LOCALE_EN]: "Bad parameters on this route",
        [LOCALE_JA]: "この道路で不適な引数がある",
    },

    search_no_results: {
        [LOCALE_EN]: "Nothing was found from this search",
        [LOCALE_JA]: "この検索で何も見つけなかった",
    },

    // -------- Any commands that go on buttons --------
    apply: {
        [LOCALE_EN]: "Apply",
        [LOCALE_JA]: "適用",
    },
    create: {
        [LOCALE_EN]: "Create",
        [LOCALE_JA]: "作成",
    },
    edit: {
        [LOCALE_EN]: "Edit",
        [LOCALE_JA]: "編集",
    },
    remove: {
        [LOCALE_EN]: "Remove",
        [LOCALE_JA]: "取り消す",
    },

    // -------- Any source and source-related things --------
    related_link_add_new: {
        [LOCALE_EN]: "Add a new Related Link",
        [LOCALE_JA]: "新しい関連リンクを追加する",
    },
    related_links: {
        [LOCALE_EN]: "Related Links",
        [LOCALE_JA]: "関連リンクス",
    },
    related_links_none: {
        [LOCALE_EN]: "There are no Related Links",
        [LOCALE_JA]: "関連リンクは存在しない",
    },

    sources: {
        [LOCALE_EN]: "Sources",
        [LOCALE_JA]: "ソースズ",
    },

    universe_tag: {
        [LOCALE_EN]: "Universe Tag",
        [LOCALE_JA]: "世界ターグ",
    },
    universe_tags: {
        [LOCALE_EN]: "Universe Tags",
        [LOCALE_JA]: "世界ターグズ",
    },
    universe_tag_edit: {
        [LOCALE_EN]: "Edit a Universe Tag",
        [LOCALE_JA]: "世界ターグを編集",
    },
    universe_tag_child_add: {
        [LOCALE_EN]: "Add a Universe Tag as a child",
        [LOCALE_JA]: "世界ターグを子として追加",
    },
    universe_tag_parent_add: {
        [LOCALE_EN]: "Add a Universe Tag as a parent",
        [LOCALE_JA]: "世界ターグを親として追加",
    },
    universe_tag_new: {
        [LOCALE_EN]: "New Universe Tag",
        [LOCALE_JA]: "新しい世界ターグ",
    },
    universe_tag_new_create: {
        [LOCALE_EN]: "Create a new Universe Tag",
        [LOCALE_JA]: "新しい世界ターグを作成して",
    },
    universe_tag_new_link: {
        [LOCALE_EN]: "See the new Universe Tag here",
        [LOCALE_JA]: "こちで新しい世界ターグを見て",
    },
};
window.CompletionTrackerLangStrings = {
    allLocales() {
        return [
            LOCALE_EN,
            LOCALE_JA,
        ];
    },
    makeI18nStrings() {
        // Since VueI18n needs them slightly differently
        const formattedStrings = {};
        for (var lang of this.allLocales()) {
            formattedStrings[lang] = {};
            for (var messageName in allLangStrings) {
                formattedStrings[lang][messageName] = allLangStrings[messageName][lang];
            }
        }
        return formattedStrings;
    },
};

window.CompletionTrackerContentLang = {
    ENGLISH: "English",
    JAPANESE: "Japanese",
    all() {
        return [
            this.ENGLISH,
            this.JAPANESE,
        ];
    },
    emptyLangMap() {
        const langMap = {};
        for (var lang of this.all()) {
            langMap[lang] = "";
        }
        return langMap;
    },
    fromLocale(locale) {
        switch (locale) {
            case LOCALE_EN:
                return "English";
            case LOCALE_JA:
                return "Japanese";
            default:
                return this.all()[0];
        }
    },
    shortName(lang) {
        switch (lang) {
            case this.ENGLISH:
                return "EN";
            case this.JAPANESE:
                return "JA";
            default:
                console.error(`Failed to find the short form for ${lang}`);
                return "{/}";
        }
    }
};

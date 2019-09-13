const LOCALE_EN = "en";
const LOCALE_JP = "jp";

// Having them closer together is nicer to see what we're using
const allLangStrings = {
    // Translations for the content Langs
    English: {
        [LOCALE_EN]: "English",
        [LOCALE_JP]: "英語",
    },
    Japanese: {
        [LOCALE_EN]: "Japanese",
        [LOCALE_JP]: "日本語",
    },

    completion_tracker: {
        [LOCALE_EN]: "Completion Tracker",
        [LOCALE_JP]: "成績トラッカー",
    },
    create: {
        [LOCALE_EN]: "Create",
        [LOCALE_JP]: "作成",
    },
    current_content_lang: {
        [LOCALE_EN]: "Current Content Language: {0}",
        [LOCALE_JP]: "現在の内容言語：{0}",
    },
    home: {
        [LOCALE_EN]: "Home",
        [LOCALE_JP]: "ホーム",
    },
    name: {
        [LOCALE_EN]: "Name",
        [LOCALE_JP]: "名",
    },
    universe_tags: {
        [LOCALE_EN]: "Universe Tags",
        [LOCALE_JP]: "世界ターグズ",
    },
    universe_tag_new: {
        [LOCALE_EN]: "New Universe Tag",
        [LOCALE_JP]: "新しい世界ターグ",
    }
};
CompletionTracker.allLocales = function() {
    return [
        LOCALE_EN,
        LOCALE_JP,
    ];
}
CompletionTracker.makeI18nStrings = function() {
    // Since VueI18n needs them slightly differently
    const formattedStrings = {};
    for (lang of CompletionTracker.allLocales()) {
        formattedStrings[lang] = {};
        for (messageName in allLangStrings) {
            formattedStrings[lang][messageName] = allLangStrings[messageName][lang];
        }
    }
    return formattedStrings;
}

CompletionTracker.contentLang = {
    all() {
        return [
            "English",
            "Japanese",
        ];
    },
    emptyMap() {
        return {
            English: "",
            Japanese: "",
        };
    },
    fromLocale(i18n) {
        switch (i18n.locale) {
            case LOCALE_EN:
                return "English";
            case LOCALE_JP:
                return "Japanese";
            default:
                return this.all()[0];
        }
    },
};

const LOCALE_EN = "en";
const LOCALE_JP = "jp";

// Having them closer together is nicer to see what we're using
const allLangStrings = {
    completion_tracker: {
        [LOCALE_EN]: "Completion Tracker",
        [LOCALE_JP]: "成績トラッカー",
    },
    home: {
        [LOCALE_EN]: "Home",
        [LOCALE_JP]: "ホーム",
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

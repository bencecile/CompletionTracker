// Makes a POST request to the URL and the stringified jsonObject in the body
// Calls onSuccess and onFail with the { data } payload
function post(url, jsonObject, onSuccess, onFail) {
    const request = new Request(url, {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify(jsonObject),
        // Make sure that we can accept and send cookies to the main server
        credentials: "same-origin",
    });

    fetch(request).then(function(response) {
        // Try to read the response text body
        response.json().then(function(jsonResponse) {
            if (jsonResponse.success) {
                onSuccess(jsonResponse.data);
            } else {
                onFail(jsonResponse.data);
            }
        }).catch(function(error) {
            // Handle the text and body read error
            onFail(error);
        });
    }).catch(function(error) {
        // Handle the request failure
        onFail(error);
    });
};

// Make a navigation holder where we can stuff things on a navigation
//  This will be good for data objects that we would otherwise have to fetch from the server
const NavHolder = {
    holder: null,
};

window.CompletionTrackerPlugin = {
    install(Vue, options) {
        const ContentLang = Vue.observable({
            current: null,
        });
        ContentLang.current = CompletionTrackerContentLang.fromLocale(options.locale);

        Vue.prototype.$setDocumentTitle = function(prefix) {
            document.title = prefix + " | " + this.$t("completion_tracker");
        };

        Object.defineProperty(Vue.prototype, "$contentLang", {
            get() { return ContentLang.current; },
            set(newLang) { ContentLang.current = newLang; },
        });

        Vue.prototype.$api = {
            readUniverseTags(reader, onSuccess, onFail) {
                post("/api/universeTags/read", reader, onSuccess, onFail);
            },
            readRootUniverseTags(onSuccess, onFail) {
                post("/api/universeTags/readRoot", {}, onSuccess, onFail);
            },
            createUniverseTag(creator, onSuccess, onFail) {
                post("/api/universeTag/create", creator, onSuccess, onFail);
            },
        };

        Vue.prototype.$states = {
            Loading: Symbol("Loading"),
            Success: Symbol("Success"),
            Error: Symbol("Error"),
        };

        Object.defineProperty(Vue.prototype, "$navHolder", {
            get() {
                const returnedHolder = NavHolder.holder;
                NavHolder.holder = null;
                return returnedHolder;
            },
            set(newHolder) { NavHolder.holder = newHolder; },
        });

        Vue.mixin({
            methods: {
                // Returns the string with the content lang or one that exists
                $langMapGet(langMap) {
                    if (!langMap[this.$contentLang]) {
                        for (var lang of CompletionTrackerContentLang.all()) {
                            if (langMap[lang]) {
                                const shortName = CompletionTrackerContentLang.shortName(lang);
                                return `${langMap[lang]} [${shortName}]`;
                            }
                        }
                        console.error("Failed to find a valid string in the map");
                        return "[NO_SUCH_STRING]";
                    }
                    return langMap[this.$contentLang];
                },
            },
        });
    },
};

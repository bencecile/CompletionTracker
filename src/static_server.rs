use rouille::{Response};

static BUNDLE_JS: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/bundle.js"));

/// Give a special debug implementation of the static server to make it cache-less
pub struct StaticServer;
impl StaticServer {
    pub fn serve_main_html(&self) -> Response {
        Response::html(r##"<!doctype html>
<html>
    <head>
        <title>PlaceHolder</title>
        <meta name="viewport" content="width=device-width, initial-scale=1, shrink-to-fit=no">

        <script>CompletionTracker = {};</script>
        <script src="/jsbundle"></script>
    </head>
    <body>
        <div id="mainPlaceholder"></div>
        <script>
            const i18n = new VueI18n({
                locale: CompletionTracker.allLocales()[0],
                fallbackLocale: CompletionTracker.allLocales()[0],
                availableLocales: CompletionTracker.allLocales(),
                messages: CompletionTracker.makeI18nStrings(),
            });
            const router = new VueRouter({
                mode: "history",
                routes: [
                    {
                        path: "/",
                        component: Vue.component("homeView")
                    }, {
                        path: "/universeTags",
                        component: Vue.component("universeTagsView")
                    }, {
                        path: "/universeTags/new",
                        component: Vue.component("universeTagsNewView")
                    },
                ],
            });
            router.beforeEach((to, from, next) => {
                document.title = `SET FOR ${to.path}`;
                next();
            });
            router.afterEach((to, from) => {
                // Clear out the navigation holder so that other things don't have to
                CompletionTracker.navigationHolder.holder = null;
            });
            const mainVue = new Vue({
                el: "#mainPlaceholder",
                router,
                i18n,
                template: `<mainView />`,
            });
        </script>
    </body>
</html>"##.to_string())
    }
    pub fn serve_bundle_js(&self) -> Response { Response::from_data("text/javascript", BUNDLE_JS) }
}

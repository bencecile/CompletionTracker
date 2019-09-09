<template>
    <div>
        <h2>{{ $t("universe_tag_new") }}</h2>
    </div>
</template>
<script>
export default {
    data() {
        return {
            // TODO Make a switch to edit other content languages for the fields
            newUniverseTag: {
                names: CompletionTracker.emptyContentLangMap(),
                descriptions: CompletionTracker.emptyContentLangMap(),
                relatedLinks: [],
            },
        };
    },
    created() {
        document.title = this.$t("universe_tag_new") + " | " + this.$t("completion_tracker");
    },
}
</script>
<style>
</style>

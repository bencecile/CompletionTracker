<template>
    <div>
        <h2>{{ $t("universe_tag_new") }}</h2>
        <contentLangPicker v-model="currentContentLang"/>

        <div>
            <label for="name">{{ $t("name") }}</label>
            <input id="name" type="text"
                v-model="newUniverseTag.names[currentContentLang]"/>
        </div>

        <button @click="createNew">{{ $t("create") }}</button>
    </div>
</template>
<script>
export default {
    data() {
        return {
            currentContentLang: CompletionTracker.contentLang.fromLocale(this.$i18n),
            isSent: false,
            newUniverseTag: {
                names: CompletionTracker.contentLang.emptyMap(),
                descriptions: CompletionTracker.contentLang.emptyMap(),
                relatedLinks: [],
                parents: [],
            },
        };
    },
    methods: {
        createNew() {
            if (isSent) { return; }
            isSent = true;

            alert(JSON.stringify(this.newUniverseTag));
            isSent = false;
        },
    },
    created() {
        document.title = this.$t("universe_tag_new") + " | " + this.$t("completion_tracker");
    },
}
</script>
<style>
</style>

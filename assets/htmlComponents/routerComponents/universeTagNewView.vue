<template>
    <div @keyup.enter="createNew">
        <h2>{{ $t("universe_tag_new") }}</h2>

        <universeTagEditor :universeTag="newUniverseTag"/>

        <button
            :disabled="isSent || createdId"
            @click="createNew">
            {{ $t("create") }}
        </button>
        <div v-if="errorMessage">{{ $t("error_occurred", [errorMessage]) }}</div>
        <router-link :to="makeNewLink()" :hidden="!createdId">
            {{ $t("universe_tag_new_link") }}
        </router-link>
    </div>
</template>
<script>
export default {
    data() {
        return {
            isSent: false,
            createdId: null,
            errorMessage: null,
            newUniverseTag: {
                names: CompletionTrackerContentLang.emptyLangMap(),
                descriptions: CompletionTrackerContentLang.emptyLangMap(),
                related_links: [],
                parents: [],
                children: [],
                related_universe_tags: [],
            },
        };
    },
    methods: {
        createNew() {
            if (this.isSent) { return; }
            this.isSent = true;
            this.errorMessage = null;

            console.log("Creating a new universe tag...");

            this.$api.createUniverseTag(this.newUniverseTag, (id) => {
                console.log(`Created universe tag ${id}`);
                this.createdId = id;
                this.isSent = false;
            }, (error) => {
                this.errorMessage = error;
                console.error(`Failed to create a Universe Tag: ${error}`);
                this.isSent = false;
            });
        },
        makeNewLink() { return `/universeTag/${this.createdId}` },
        addNewRelatedLink() {
            // Push a new link with description
            this.related_links.push(["", ""]);
        },
    },
    created() {
        this.$setDocumentTitle(this.$t("universe_tag_new"));
    },
}
</script>
<style>
</style>

<template>
    <div class="relatedLinkInputContainer">
        <button @click="addNewRelatedLink">{{ $t("related_link_add_new") }}</button>
        <div v-if="value.length > 0" class="relatedLinkInputRow">
            <div>URL</div>
            <div>{{ $t("description") }}</div>
        </div>
        <div v-for="(link, i) in value" :key="i" class="relatedLinkInputRow">
            <input type="text" v-model="link[0]">
            <input type="text" v-model="link[1][$contentLang]">
            <button @click="removeRelatedLink(i)">{{ $t("remove") }}</button>
        </div>
    </div>
</template>
<script>
export default {
    props: {
        value: { type: Array, required: true },
    },
    methods: {
        addNewRelatedLink() {
            // Push a new link with description
            this.value.push(["", CompletionTrackerContentLang.emptyLangMap()]);
        },
        removeRelatedLink(index) {
            this.$emit("input", this.value.filter((_, i) => i != index))
        },
    },
}
</script>
<style>
.relatedLinkInputContainer {
    display: grid;
    grid-template-columns: 1fr;
    row-gap: 0.25em;
}
.relatedLinkInputRow {
    display: grid;
    grid-template-columns: 15em 20em min-content;
}
</style>

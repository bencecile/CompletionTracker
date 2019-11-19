<template>
    <div>
        <div v-if="relatedLinks.length > 0" class="relatedLinksContainer">
            <div class="grid-centerH tableTitle">{{ $t("related_links") }}</div>
            <div class="relatedLinksRow">
                <div>URL</div>
                <div>{{ $t("description") }}</div>
            </div>
            <div v-for="(link, i) in relatedLinks" :key="i" class="relatedLinksRow">
                <div>
                    <a :href="link.url" target="_blank">{{ link.url }}</a>
                </div>
                <div>{{ $langMapGet(link.descriptions) }}</div>
            </div>
        </div>
        <div v-if="relatedLinks.length == 0">
            {{ $t("related_links_none") }}
        </div>
    </div>
</template>
<script>
export default {
    props: {
        relatedLinks: { type: Array, required: true },
    },
}
</script>
<style>
.relatedLinksContainer {
    display: grid;
    grid-template-columns: 1fr;
    row-gap: 0.25em;
}
.relatedLinksRow {
    display: grid;
    grid-template-columns: 15em 20em;
    border-bottom: 1px solid black;
}
</style>

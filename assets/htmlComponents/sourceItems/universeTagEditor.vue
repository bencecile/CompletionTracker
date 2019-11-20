<template>
    <div class="universeTagEditor">
        <div class="formGrid2Col">
            <label for="name">{{ $t("name") }}</label>
            <input id="name" type="text"
                v-model="universeTag.names[$contentLang]"/>
            <label for="description">{{ $t("description") }}</label>
            <input id="description" type="text"
                v-model="universeTag.descriptions[$contentLang]"/>
        </div>

        <relatedLinkInput v-model="universeTag.related_links" />

        <div class="universeTagParentsChildrenContainer">
            <itemPickerList :itemDescription="$t('universe_tag_parent_add')"
                :itemType="$itemTypes.UniverseTag"
                @input="universeTag.parents = $event"/>
            <itemPickerList :itemDescription="$t('universe_tag_child_add')"
                :itemType="$itemTypes.UniverseTag"
                @input="universeTag.children = $event"/>
        </div>
    </div>
</template>
<script>
export default {
    props: {
        universeTag: { type: Object, required: true },
    },
}
</script>
<style>
.universeTagEditor {
    display: grid;
    grid-template-columns: 1fr;
    row-gap: 0.75em;
}
.universeTagParentsChildrenContainer {
    display: grid;
    grid-template-columns: 1fr 1fr;
    column-gap: 1em;
    align-items: start;
}
</style>

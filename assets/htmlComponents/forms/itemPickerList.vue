<template>
    <div class="itemPickerList">
        <div class="grid-centerH tableTitle">{{ itemDescription }}</div>
        <div v-for="(item, i) in fetchedItems" :key="item.id" class="itemPickerListRow">
            <div>{{ $langMapGet(item.names) }}</div>
            <button @click="removeFetchedItem(i)">{{ $t("remove") }}</button>
        </div>
        <itemPicker @input="pushFetchedItem" :itemType="itemType"/>
    </div>
</template>
<script>
export default {
    // Emits the IDs of the picked items as an "input" event
    props: {
        itemDescription: { type: String, required: true },
        itemType: { type: String, required: true },
    },
    data() {
        return {
            fetchedItems: [],
        };
    },
    methods: {
        pushFetchedItem(item) {
            this.fetchedItems.push(item);
            this.emitItemIds();
        },
        removeFetchedItem(index) {
            this.fetchedItems = this.fetchedItems.filter((_, i) => i != index);
            this.emitItemIds();
        },
        emitItemIds() {
            this.$emit("input", this.fetchedItems.map(mappedItem => mappedItem.id));
        },
    },
}
</script>
<style>
.itemPickerList {
    display: grid;
    grid-template-columns: 1fr;
    row-gap: 0.1em;
}
.itemPickerListRow {
    display: grid;
    grid-template-columns: 1fr min-content;
    row-gap: 0.25em;
    column-gap: 0.25em;
}
</style>

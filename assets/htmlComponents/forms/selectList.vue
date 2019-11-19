<template>
    <div class="selectList"
        @keydown.down.stop="navWrap(-1)" @keydown.up.stop="navWrap(1)"
        @keydown.enter.stop="select()">
        <div v-for="(text, i) in listTexts" :key="i">
            <div :class="['selectListItem', highlightClass(i)]"
                @mouseenter="navTo(i)" @mousedown="select()">
                {{text}}
            </div>
        </div>
    </div>
</template>
<script>
export default {
    props: {
        listValues: Array,
        listTexts: Array,
    },
    data() {
        return {
            index: 0,
        };
    },
    methods: {
        highlightClass(i) { return this.index == i ? "highlight" : ""; },
        navWrap(amount) {
            let oldIndex = this.index;
            this.navTo(this.index + amount);
            if (this.index === oldIndex) {
                if (amount > 0) {
                    this.index = 0;
                } else if (amount < 0) {
                    this.index = this.listValues.length - 1;
                }
            }
        },
        navTo(i) {
            if (i >= 0 && i < this.listValues.length) {
                this.index = i;
            }
        },
        select() {
            this.$emit("input", this.listValues[this.index]);
        },
    },
}
</script>
<style>
.selectList {
    display: grid;
    width: 100%;
    grid-template-columns: 1fr;
    row-gap: 0.25em;
}
.selectListItem {
    width: calc(100% - 0.2em);
    padding: 0.1em;
}
</style>

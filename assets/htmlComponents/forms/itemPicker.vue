<template>
    <div class="itemPicker"
        @keydown.down="navWrap(-1)" @keydown.up="navWrap(1)"
        @keyup.enter.stop="selectChosenItem()">
        <input type="text" :value="searchQuery" @input="updateQuery($event.target.value)"
            @focus="hasFocus = true" @blur="closeSelectList()">
        <div v-if="state != $states.Nothing" :class="['itemPickerResults', focusClass]">
            <div v-if="state === $states.Loading">{{ $t("loading") }}</div>
            <div v-if="state === $states.Error">{{ $t("error_occurred", [errorMessage]) }}</div>
            <div v-if="state === $states.Success && resultsWithItems.length != 0"
                class="itemPickerSelectList">
                <div v-for="(text, i) in selectListTexts" :key="i"
                    :class="['itemPickerSelectListItem', highlightClass(i)]"
                    @mousedown="clickedOnList = true"
                    @mouseenter="navTo(i)" @click="selectChosenItem()">
                    {{text}}
                </div>
            </div>
            <div v-else>
                {{ $t("search_no_results") }}
            </div>
        </div>
    </div>
</template>
<script>
export default {
    props: {
        itemType: { type: String, required: true },
    },
    data() {
        return {
            searchQuery: "",
            state: this.$states.Nothing,
            errorMessage: null,
            resultsWithItems: [],
            timerHandle: null,
            hasFocus: false,
            clickedOnList: false,

            selectIndex: 0,
        };
    },
    computed: {
        focusClass() { return this.hasFocus ? "" : "disappear"; },
        selectListTexts() {
            return this.resultsWithItems.map(result => this.$langMapGet(result.item.names));
        },
    },
    methods: {
        updateQuery(query) {
            if (this.timerHandle) {
                clearTimeout(this.timerHandle);
            }
            this.hasFocus = true;
            this.searchQuery = query;
            if (!this.searchQuery) {
                this.resetState();
                this.timerHandle = null;
            } else {
                this.timerHandle = setTimeout(() => this.fetchSearchResults(), 300);
            }
        },
        fetchSearchResults() {
            this.$searchAndFetch(this.searchQuery, this.itemType, (resultsWithItems) => {
                this.resultsWithItems = resultsWithItems;
                this.state = this.$states.Success;
                this.selectIndex = 0;
                this.timerHandle = null;
            }, (errorMessage) => {
                this.setErrorState(errorMessage);
                this.timerHandle = null;
            });
        },
        selectChosenItem() {
            if (this.resultsWithItems.length > 0) {
                this.$emit("input", this.resultsWithItems[this.selectIndex].item);
                this.resetState();
            }
        },
        resetState() {
            this.searchQuery = "";
            this.state = this.$states.Nothing,
            this.errorMessage = null;
            this.resultsWithItems = [];
            this.hasFocus = false;
            this.clickedOnList = false;
            this.selectIndex = 0;
        },
        setErrorState(errorMessage) {
            this.state = this.$states.Error;
            this.errorMessage = errorMessage;
            console.error(this.errorMessage);
        },

        closeSelectList() {
            if (this.clickedOnList) {
                this.clickedOnList = false;
            } else {
                this.hasFocus = false;
            }
        },
        highlightClass(i) { return this.selectIndex == i ? "highlight" : ""; },
        navWrap(amount) {
            let oldIndex = this.selectIndex;
            this.navTo(this.selectIndex + amount);
            if (this.selectIndex === oldIndex) {
                if (amount > 0) {
                    this.selectIndex = 0;
                } else if (amount < 0) {
                    this.selectIndex = this.resultsWithItems.length - 1;
                }
            }
        },
        navTo(i) {
            if (i >= 0 && i < this.resultsWithItems.length) {
                this.selectIndex = i;
            }
        },
    },
}
</script>
<style>
.itemPicker {
    position: relative;
    width: 100%;
}
.itemPickerResults {
    width: 100%;
    max-height: 10em;
    position: absolute;
    background: white;
    box-shadow: 2px 2px 3px 1px #222A;
}

.itemPickerSelectList {
    display: grid;
    width: 100%;
    grid-template-columns: 1fr;
    row-gap: 0.25em;
}
.itemPickerSelectListItem {
    width: calc(100% - 0.2em);
    padding: 0.1em;
}
</style>

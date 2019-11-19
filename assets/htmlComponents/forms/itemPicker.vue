<template>
    <div class="itemPicker">
        <input type="text" :value="searchQuery" @input="updateQuery"
            @focus="hasFocus = true" @blur="hasFocus = false">
        <div v-if="state != $states.Nothing" :class="['itemPickerResults', focusClass]">
            <div v-if="state === $states.Loading">{{ $t("loading") }}</div>
            <div v-if="state === $states.Error">{{ $t("error_occurred", [errorMessage]) }}</div>
            <selectList v-if="state === $states.Success && resultsWithItems.length != 0"
                :listValues="listValues" :listTexts="listTexts"
                @input="selectChosenItem"/>
            <div v-else>
                {{ $t("search_no_results") }}
            </div>
        </div>
    </div>
</template>
<script>
export default {
    // TODO Implement the select list inline so that keys can work
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
        };
    },
    computed: {
        focusClass() { return this.hasFocus ? "" : "disappear"; },
        listValues() { return this.resultsWithItems.map(result => result.item); },
        listTexts() {
            return this.resultsWithItems.map(result => this.$langMapGet(result.item.names));
        },
    },
    methods: {
        updateQuery(event) {
            if (this.timerHandle) {
                clearTimeout(this.timerHandle);
            }
            this.hasFocus = true;
            this.searchQuery = event.target.value;
            if (!this.searchQuery) {
                this.resetState();
                this.timerHandle = null;
            } else {
                this.timerHandle = setTimeout(() => {
                    this.fetchSearchResults();
                }, 300);
            }
        },
        fetchSearchResults() {
            this.$searchAndFetch(this.searchQuery, this.itemType, (resultsWithItems) => {
                this.resultsWithItems = resultsWithItems;
                this.state = this.$states.Success;
            }, (errorMessage) => {
                this.setErrorState(errorMessage);
            });
        },
        selectChosenItem(item) {
            this.$emit("input", item);
            this.resetState();
        },
        resetState() {
            this.searchQuery = "";
            this.state = this.$states.Nothing,
            this.errorMessage = null;
            this.resultsWithItems = [];
            this.hasFocus = false;
        },
        setErrorState(errorMessage) {
            this.state = this.$states.Error;
            this.errorMessage = errorMessage;
            console.error(this.errorMessage);
        }
    },
}
</script>
<style>
.itemPicker {
    position: relative;
    width: 100%;
}
.itemPicker > input {
    width: calc(100% - 2px);
    padding: 0;
    border: 1px solid black;
}
.itemPickerResults {
    width: 100%;
    max-height: 10em;
    position: absolute;
    background: white;
    box-shadow: 2px 2px 3px 1px black;
}
</style>

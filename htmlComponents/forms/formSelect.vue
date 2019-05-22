<template>
    <div class="formSelect theme-light1">
        <input :id="id" class="formSelect-text" type="text" @input="updateText">
        <svg class="formSelect-arrow" xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 100 100">

        </svg>
        <div class="formSelect-options theme-light1">
            <div v-for="option in options" :key="option[1]">
                {{option[1]}}
            </div>
        </div>
    </div>
</template>

<script>
export default {
    data() {
        var text = "";
        var value = "";
        if (this.defaultSelection) {
            [value, text] = this.options[this.defaultSelection];
        }
        return {
            // The text that's showing
            text,
            // The actual value of the select
            value,
            // The selected option as an index
            selected: this.defaultSelection,
        };
    },
    methods: {
        // Updates the text with what's that's in the input field
        updateText() {

        },
    },
    props: {
        // The list of options to choose from
        options: { type: Array, required: true },
        // The default option index to show by default, if any
        defaultSelection: { type: Number, default: null },
        // The ID of the input field to use
        id: { type: String, required: true },
    },
}
</script>

<style>
.formSelect {
    position: relative;
    display: flex;
    padding-right: 0.5em;
}
.formSelect-text {
    width: 10em;
}
.formSelect-arrow {
    border: 1px solid black;
    box-shadow: 0 4em 8em 0 #0003;
    width: 1.5em;
}
.formSelect-options {
    position: absolute;
    display: none;
    top: 1.5em;
    border-top: 1px solid #222;
    max-height: 10em;
    width: 10em;
    z-index: 1;
    overflow-y: scroll;
}
</style>

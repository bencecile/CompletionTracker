<template>
    <div class="formArea theme-light2"
        :class="{ 'focus-input': inFocus }"
        :style="{ height: totalHeight }">
        <textarea class="formArea-text"
            :value="value" :id="id"
            :style="{ height: totalHeight, 'word-break': breakChar ? 'break-all' : 'normal' }"
            @input="getInput"
            @keydown.enter="suppressEnter" @keyup.enter="suppressEnter"
            @focus="setFocus(true)" @blur="setFocus(false)">
        </textarea>
    </div>
</template>

<script>
export default {
    props: {
        // The ID of the textarea element
        id: { type: String, required: true },
        // The thing passed in when using v-model
        value: { type: String, required: true },
        // If we want to disable any new lines
        singleLine: { type: Boolean, default: false },
        // If we want to break a every character, or use the default behaviour
        // This is good for URLs or other things that aren't supposed to have spaces
        breakChar: { type: Boolean, default: false },
    },
    data() {
        return {
            // Whether or not the textarea field has focus
            inFocus: false,
            // The number of lines we need to create
            lines: 1,
        };
    },
    computed: {
        totalHeight() { return `${this.lines * 1.5}em`; },
    },
    methods: {
        // Uses the input from the text area
        getInput(event) {
            // Remove any newlines that may have been entered if we are in singleLine mode
            var newValue = event.target.value;
            if (this.singleLine) {
                newValue = newValue.replace("\n", "");
            }
            this.setText(newValue);

            this.findNeededLines();
        },
        setText(newText) {
            this.textArea.value = newText;
            // Let the parent know about the value change
            this.$emit("input", newText);
        },
        // We may have to stop the enter key from propagating
        suppressEnter(event) {
            if (!this.singleLine) { event.stopPropagation(); }
        },
        // Finds the exact number of lines the text area needs to be to fit all of the text
        findNeededLines() {
            // Find any new lines and create lines first based on those
            const clientWidth = this.textArea.clientWidth;
            this.lines = this.textArea.value.split("\n").reduce((lines, lineString) => {
                // Add a lines since this IS a new line
                lines += 1;
                // Find out any extra lines from each line width
                const totalWidth = this.stringMeasurer.measureString(lineString);
                // Find out how many lines we would need to fit all of the text
                lines += Math.floor(totalWidth / clientWidth);
                return lines;
            }, 0);
        },
        setFocus(newFocus) { this.inFocus = newFocus; }
    },
    mounted() {
        // Get the text area for later use
        this.textArea = this.$el.querySelector(".formArea-text");

        this.stringMeasurer = createStringMeasurer(this.textArea);

        // Since there may be pre-existing text, we need to re-layout the field
        this.findNeededLines();
    },
}
</script>

<style>
.formArea {
    width: calc(100% - 0.5em);
    padding: 0 0.25em;
}
.formArea-text {
    border: none;
    outline: none;
    color: inherit;
    background: inherit;
    font: inherit;
    resize: none;
    overflow: hidden;
    padding: 0;
    margin: 0;

    width: 100%;
    line-height: 1.5em;
}
</style>

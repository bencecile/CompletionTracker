<template>
    <div class="formSelect text-line theme-light2"
        :class="{ 'focus-input': inFocus }"
        @focusin="setFocus(true)" @focusout="setFocus(false)"
        @keydown.down="moveSelection(true)" @keydown.up="moveSelection(false)"
        @keyup.enter="makeSelection">
        <input class="formSelect-text" type="text"
            :id="id" v-model="text" @input="updateTyping">
        <svg class="formSelect-expander text" viewBox="0 0 100 100"
            :class="{ 'theme-light5': inFocus }"
            @click="buttonFocus">
            <!-- Create a menu-looking thing -->
            <!-- These are the small dots on the left side -->
            <rect x="20" y="25" width="10" height="10" />
            <rect x="20" y="45" width="10" height="10" />
            <rect x="20" y="65" width="10" height="10" />

            <!-- These are the larger bars on the right side -->
            <rect x="40" y="25" width="40" height="10" />
            <rect x="40" y="45" width="40" height="10" />
            <rect x="40" y="65" width="40" height="10" />    
        </svg>
        <div class="formSelect-options theme-light2"
            tabindex="-1"
            :class="{ closed: !inFocus }">
            <div v-for="(option, i) in filteredOptions" :key="option.originalIndex"
                class="text-line"
                :class="{ 'theme-light5': selected === option.originalIndex }"
                @mouseover="setSelection(i)" @click="makeSelection">
                {{option.text}}
            </div>
        </div>
    </div>
</template>

<script>
export default {
    props: {
        // The list of options to choose from
        options: { type: Array, required: true },
        // The default option index to show by default, if any
        defaultValue: { type: String, default: null },
        // The ID of the input field to use
        id: { type: String, required: true },
        value: { type: String, required: true },
    },
    data() {
        var text = "";
        var selected = 0;
        if (this.defaultValue) {
            // Find the value in the options
            selected = this.options.findIndex((option) => option[0] === this.defaultValue);
            if (selected === -1) {
                console.error("Failed to find the default value", this.defaultValue);
            }
            text = this.options[selected][1];
        }
        return {
            // The text that's showing
            text,
            // The selected option as an index
            selected,
            // A flag for when the user starts typing
            // This will allow us to show the full list at first, even if there was a default
            startedTyping: false,
            // If the form select is currently in focus
            inFocus: false,
            // Stop the button from changing focus
            buttonLock: false,
            // The timeout function to unlock the gate
            focusTracker: new TimeoutTracker(function(data) { data.buttonLock = false; }),
        };
    },
    computed: {
        // Filter the list that is being shown in the select
        filteredOptions() {
            // The lower case version of the text so we don't have to compute it every time
            var lowerText = this.text.toLowerCase();
            // Show the entire list if the user hasn't started typing yet
            if (!this.startedTyping) {
                // This will match everything
                lowerText = "";
            }

            // Keep all of the options that start with the typed text
            return this.options.map((option, i) => {
                // Return an object instead
                return {
                    text: option[1],
                    originalIndex: i,
                };
            }).filter((option) =>
                option.text.toLowerCase().startsWith(lowerText)
            );
        },
    },
    methods: {
        updateTyping(event) {
            // We have started typing
            if (!this.startedTyping) { this.startedTyping = true; }
            // We also definitely have focus since we are typing
            this.setFocus(true);

            // If the selected option is no longer in the filtered ones, set it to 0
            const filtered = this.filteredOptions;
            const found = filtered.some((option) => option.originalIndex === this.selected);
            if (!found && filtered[0]) {
                this.selected = filtered[0].originalIndex;
            }

            // Fire off the custom input event
            this.$emit('input', event.target.value);
        },
        // Sets the focus on the select form
        setFocus(newFocus) {
            // Only try to set the focus if it will change
            if (this.inFocus === newFocus) { return; }

            // Lock the focus gate
            this.buttonLock = true;

            this.inFocus = newFocus;
            if (!this.inFocus) {
                // Make sure the text in the box is correct
                // The user could have typed but without making a selection the value is still old
                this.text = this.options[this.selected][1];
                // Reset the typing status so that everything can be seen if we re-focus
                this.startedTyping = false;
            }

            // Unlock the focus gate some time later
            this.focusTracker.setTimeout(300, this);
        },
        buttonFocus() {
            // Only proceed if the gate is unlocked
            if (this.buttonLock) { return; }
            this.setFocus(!this.inFocus);
            // Set the input field to actually have focus
            this.$el.querySelector(".formSelect-text").focus();
        },
        // Moves the selection by a boolean (true is plus, false is minus)
        moveSelection(plus) {
            // Get the filtered options that are currently being displayed
            const filtered = this.filteredOptions;
            
            // Do nothing if the filtered list is empty
            if (filtered.length === 0) { return; }

            // Find the filtered index of the selected option
            var filteredIndex = filtered.findIndex((option) =>
                option.originalIndex === this.selected);
            // Find the index of the next option
            filteredIndex += (plus ? 1 : -1);
            // Don't go over the bounds
            if (filteredIndex < 0) {
                // Grab the 0th entry
                filteredIndex = 0;
            } else if (filteredIndex >= filtered.length) {
                // Grab the last entry
                filteredIndex = filtered.length - 1;
            }

            this.selected = filtered[filteredIndex].originalIndex;

            // Make sure the selected element is in view
            const selectedElement = this.$el.querySelector(".formSelect-options")
                .children[filteredIndex];
            // Set the scroll to be able to show 2 elements above the selected
            const verticalScroll = selectedElement.clientHeight * (filteredIndex - 2);
            selectedElement.parentElement.scrollTo(0, verticalScroll);
        },
        // Forcibly sets the selection to the given filtered element index
        setSelection(filteredIndex) {
            this.selected = this.filteredOptions[filteredIndex].originalIndex;
        },
        // Makes the selection, making the value now associated to the selected index
        makeSelection(event) {
            // If this was started from an event, we want to stop the propagation
            if (event && this.inFocus) { event.stopPropagation(); }
            this.sendInputEvent();
            // Remove the focus to show that something has been picked
            this.setFocus(false);
        },
        // Sends the input event to the parent so that the model value can change
        sendInputEvent() {
            this.$emit("input", this.options[this.selected][0]);
        },
    },
    mounted() {
        // Make sure that the modelled value has something
        this.sendInputEvent();
    },
}
</script>

<style>
.formSelect {
    position: relative;
    display: flex;
}
.formSelect-text {
    width: calc(100% - 0.5em);
    padding: 0 0.25em;
}
.formSelect-expander {
    width: 1.5em;
    fill: currentColor;
    transition: all 300ms ease;
}
.formSelect-options {
    position: absolute;
    top: 100%;
    z-index: 1;
    border-top: 1px solid #222;
    box-shadow: 0 0.25em 0.5em 0 rgba(0,0,0,0.2);
    max-height: 10em;
    width: 100%;
    overflow-y: auto;

    transition: all 300ms ease;
}
.formSelect-options.closed { transform: scaleY(0); }
.formSelect-options > div {
    padding-left: 0.5em;
}
</style>

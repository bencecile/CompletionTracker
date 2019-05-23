<template>
    <div class="formSelect theme-light1"
        @focusin="setFocus(true)" @focusout="setFocus(false)"
        @keydown.down="moveSelection(true)" @keydown.up="moveSelection(false)"
        @keydown.enter="makeSelection">
        <input :id="id" class="formSelect-text" type="text"
            v-model="text" @input="updateTyping">
        <svg class="formSelect-expander" xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 100 100"
            @click="buttonFocus"
            :class="{ 'theme-primary': inFocus }">
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
        <div class="formSelect-options theme-light1"
            tabindex="-1"
            :class="{ closed: !inFocus }">
            <div v-for="(option, i) in filteredOptions" :key="option.originalIndex"
                :class="{ 'theme-primary': selected === option.originalIndex }"
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
    },
    data() {
        var text = "";
        var value = "";
        var selected = 0;
        if (this.defaultValue) {
            // Find the value in the options
            selected = this.options.findIndex((option) => option[0] === this.defaultValue);
            if (!selected) {
                console.error("Failed to find the default value", this.defaultValue);
            }
            [value, text] = this.options[selected];
        }
        return {
            // The text that's showing
            text,
            // The actual value of the select
            value,
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
        },
        // Sets the focus on the select form
        setFocus(newFocus) {
            // Only try to set the focus if it will change
            if (this.inFocus === newFocus) { return; }

            // Lock the focus gate
            this.buttonLock = true;

            this.inFocus = newFocus;

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
        makeSelection() {
            [this.value, this.text] = this.options[this.selected];
            // Remove the focus to show that something has been picked
            this.inFocus = false;
            // Reset the typing status of this field so that everything can be seen again
            this.startedTyping = false;
        },
    },
}
</script>

<style>
.formSelect {
    position: relative;
    display: flex;
}
.formSelect-text {
    width: 10em;
}
.formSelect-expander {
    width: 1.5em;
    fill: currentColor;
}
.formSelect-options {
    position: absolute;
    z-index: 1;
    top: 1.5em;
    border-top: 1px solid #222;
    box-shadow: 0 0.25em 0.5em 0 rgba(0,0,0,0.2);
    max-height: 10em;
    width: 100%;
    overflow-y: scroll;

    transition: all 300ms ease;
}
.formSelect-options.closed { transform: scaleY(0); }
.formSelect-options > div {
    padding-left: 0.5em;
}
</style>

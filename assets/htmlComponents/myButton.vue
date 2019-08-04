<template>
    <button class="btn"
        :class="[currentTheme, { 'focus-input': canShowFocus }]"
        @mousedown="setPressed(true)" @mouseup="setPressed(false)"
        @keydown.enter="setPressed(true)" @keyup.enter="setPressed(false)"
        @mouseover="setHover(true)" @mouseleave="setHover(false)"
        @focus="setFocus(true)" @blur="setFocus(false)">
        <slot></slot>
    </button>
</template>

<script>
export default {
    props: {
        // Whether or not the focus of the button is **really** shown
        showFocus: { type: Boolean, default: false },
        // The theme to use for the button. theme-primary is the highest you should go
        theme: { type: String, default: "theme-primary" },
    },
    data() {
        return {
            // A button can have focus if it's been tabbed onto
            hasFocus: false,
            // The mouse is hovering over the button
            isHover: false,
            // The mouse is pressing the button
            isPressed: false,
        };
    },
    computed: {
        // The hover theme is one step lighter than the base theme
        hoverTheme() { return this.lighterTheme(this.theme); },
        // Set the current theme based on the press and hover state
        currentTheme() {
            // Check for a press first
            // Use a lighter theme than the hover one to distinguish them
            if (this.isPressed) { return this.lighterTheme(this.hoverTheme); }
            // Check the focus before hover so that hover can't "overwrite" it
            if (this.hasFocus && this.showFocus) { return this.hoverTheme; }
            if (this.isHover) { return this.hoverTheme; }
            // Just return the base theme since there is no hover or click
            return this.theme;
        },
        canShowFocus() {
            // Never show the focus if that's what the man upstairs says
            if (!this.showFocus) { return false; }
            return this.isHover || this.hasFocus;
        }
    },
    methods: {
        // Gets the next lighter theme than the one given
        lighterTheme(theme) { return BACKGROUND_MAP[theme]; },
        setFocus(newFocus) { this.hasFocus = newFocus; },
        setHover(newHover) { this.isHover = newHover; },
        setPressed(newPressed) {
            this.isPressed = newPressed;
            // Only send the new event on mouse up
            if (!this.isPressed) {
                // We can now send the click event
                this.$emit("click", true);
            }
        },
    },
}
</script>

<style>
</style>

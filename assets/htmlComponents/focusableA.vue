<template>
    <a :href="href"
        @focus="focus(true)" @blur="focus(false)"
        @mouseenter="hover(true)" @mouseleave="hover(false)"
        :class="highlightTheme">
        <slot/>
    </a>
</template>

<script>
export default {
    props: {
        // The link to navigate to
        href: { type: String, required: true },
    },
    data() {
        return {
            // If the element is in focus
            inFocus: false,
            // If the link is hovered
            // These two are separate so that mousing off a focused link doesn't un-highlight it
            isHover: false,
        }
    },
    computed: {
        // The highlight theme to use
        highlightTheme() {
            if (this.inFocus || this.isHover) { return "theme-light5"; }
            // Don't use a theme if the link shouldn't be highlighted
            return "";
        },
    },
    methods: {
        // Highlight the element that is being focused
        focus(newFocus) { this.inFocus = newFocus; },
        // Hovers over the link text
        hover(newHover) { this.isHover = newHover; }
    },
}
</script>

<style>
</style>

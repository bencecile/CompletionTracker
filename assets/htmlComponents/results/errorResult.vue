<template>
    <div class="errorResult boxshadow">
        <svg class="icon" viewBox="-50 -50 100 100">
            <circle cx="0" cy="0" r="45" fill="none" stroke-width="5"/>
            <line x1="-25" y1="-25" x2="25" y2="25" stroke-width="5"/>
            <line x1="-25" y1="25" x2="25" y2="-25" stroke-width="5"/>
        </svg>
        <div class="errorResult-text">{{text}}</div>
    </div>
</template>

<script>
export default {
    props: {
        // The text to show in the error field
        text: { type: String, required: true },
    },
}
</script>

<style>
.errorResult {
    padding: 0.5em 0.5em;

    display: grid;
    grid-template: 1fr / max-content 1fr;
    place-items: center center;
}
.errorResult > svg { width: 3em; }
</style>

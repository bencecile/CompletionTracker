<template>
    <div class="successResult boxshadow">
        <svg class="icon" viewBox="-50 -50 100 100">
            <circle cx="0" cy="0" r="45" fill="none" stroke-width="5"/>
            <polyline points="-30,10 -10,30 25,-25" stroke-width="5" fill="none"/>
        </svg>
        <div class="successResult-text">{{text}}</div>
    </div>
</template>

<script>
export default {
    props: {
        // The text to show in the success field
        text: { type: String, required: true },
    },
}
</script>

<style>
.successResult {
    padding: 0.5em 0.5em;

    display: grid;
    grid-template: 1fr / max-content 1fr;
    place-items: center center;
}
.successResult > svg { width: 3em; }
</style>

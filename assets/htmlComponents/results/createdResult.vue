<template>
    <div class="createdResult boxshadow">
        <svg class="icon" viewBox="0 0 100 100">
            <defs>
                <polyline id="checkmark" points="0,72.5 25,97.5 87.5,35"
                    stroke-width="5" fill="none"/>
            </defs>
            <mask id="checkmarkMask">
                <rect x="0" y="0" width="100" height="100" fill="white"/>
                <use href="#checkmark" transform="translate(0 -5)" stroke="black"/>
                <use href="#checkmark" transform="translate(0 5)" stroke="black"/>
                <use href="#checkmark" transform="translate(-5 0)" stroke="black"/>
                <use href="#checkmark" transform="translate(5 0)" stroke="black"/>
            </mask>
            <g mask="url(#checkmarkMask)">
                <rect x="15" y="5" width="70" height="90" rx="10"
                    stroke-width="5" fill="none"/>
                <rect x="30" y="15" width="40" height="5"/>
                <rect x="30" y="35" width="40" height="5"/>
                <rect x="30" y="55" width="40" height="5"/>
                <rect x="30" y="75" width="40" height="5"/>
            </g>
            <use href="#checkmark"/>
        </svg>
        <div class="createdResult-text">{{text}}</div>
        <focusableA :href="link">{{linkText}}</focusableA>
        <myButton :showFocus="true" @click="$emit('undo')">{{Global.uiStrings.undo.current()}}</myButton>
    </div>
</template>

<script>
export default {
    props: {
        // The text to show in the loading field
        text: { type: String, required: true },
        // The link to the newly created thing
        link: { type: String, required: true },
        // The text to show for the link
        linkText: { type: String, required: true},
    },
}
</script>

<style>
.createdResult {
    padding: 0.5em 0.5em;

    display: grid;
    grid-template: 1fr 1fr / max-content 1fr 1fr;
    row-gap: 0.5em;
    place-items: center center;
}
.createdResult > svg {
    width: 3em;
    height: 3em;

    stroke: currentColor;
    fill: currentColor;

    grid-area: 1 / 1 / 3 / 2;
}
.createdResult-text {
    grid-column: span 2;
}
</style>

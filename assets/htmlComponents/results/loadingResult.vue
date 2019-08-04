<template>
    <div class="loadingResult boxshadow">
        <svg class="icon" viewBox="-50 -50 100 100">
            <mask id="loadingMask">
                <path d="M 0,0
                    c 5,0 5,5 0,5
                    c -10,0 -10,-10 0,-10
                    c 20,0 20,20 0,20
                    c -30,0 -30,-30 0,-30
                    c 40,0 40,40 0,40
                    c -50,0 -50,-50 0,-50"
                    stroke-width="2" stroke="white" fill="none"/>
                <path d="M 0,-35
                    C 30,-35 40,-20 35,-15
                    C 45,10 40,20 30,20
                    C 10,45 -10,45 -10,35
                    C -45,30 -45,15 -40,10
                    C -50,-20 -40,-20 -30,-25
                    C -10,-40 -5,-45 0,-35"
                    stroke-width="2" stroke="white" fill="none"/>
            </mask>
            <circle cx="0" cy="0" r="50" mask="url(#loadingMask)" stroke="none">
                <animate attributeName="r"
                    keyTime="0; 0.45; 0.55; 1" values="0; 50; 50; 0"
                    dur="3s" repeatCount="indefinite"   />
            </circle>
        </svg>
        <div class="loadingResult-text">{{text}}</div>
    </div>
</template>

<script>
export default {
    props: {
        // The text to show in the loading field
        text: { type: String, required: true },
    },
}
</script>

<style>
.loadingResult {
    padding: 0.5em 0.5em;

    display: grid;
    grid-template: 1fr / max-content 1fr;
    place-items: center center;
}
.loadingResult > svg { width: 3em; }
</style>

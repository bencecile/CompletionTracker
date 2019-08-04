<template>
    <div class="dropdownMenu theme-primary"
        @focusin="focus(true)" @focusout="focus(false)"
        @mouseenter="fold(true)" @mouseleave="fold(false)"
        @mouseup="clickFold">
        <focusableA class="dropdownMenu-link" :href="href">{{title}}</focusableA>
        <div class="dropdownMenu-arrowBox"
            :class="{ open: isFolderOpen }">
            <svg class="dropdownMenu-arrow text" viewBox="0 0 100 100">
                <polyline points="30,40 50,60 70,40" fill="none"/>
            </svg>
        </div>
        <div class="dropdownMenu-folder theme-primary"
            :class="{ closed: !isFolderOpen }">
            <slot></slot>
        </div>
    </div>
</template>

<script>
export default {
    props: {
        title: { type: String, required: true },
        href: { type: String, required: true },
    },
    data() {
        return {
            // If the element is in focus
            inFocus: false,
            isOpen: false,
            // Have a lock for how often the folder can be folded
            clickLock: false,
            clickTimer: new TimeoutTracker(function(dropdownMenu) {
                dropdownMenu.clickLock = false;
            }),
        }
    },
    computed: {
        // Checks if the folder should be open
        isFolderOpen() { return this.isOpen || this.inFocus; },
    },
    methods: {
        // Allow clicking to close it
        clickFold() {
            // Don't fold if locked
            if (this.clickLock) { return; }
            this.setClickLock();

            this.fold(!this.isOpen);
        },
        // Sets the click lock
        setClickLock() {
            this.clickLock = true;
            this.clickTimer.setTimeout(300, this);
        },
        fold(newOpen) {
            // Lock the button from closing so soon after opening
            this.setClickLock();
            this.isOpen = newOpen;
        },
        // Highlight the element that is being focused
        focus(newFocus) { this.inFocus = newFocus; },
    },
}
</script>

<style>
.dropdownMenu {
    position: relative;
    display: flex;
    cursor: pointer;
}
.dropdownMenu-link {
    height: 2em;
    padding: 0 0.25em;
    text-align: center;
}
.dropdownMenu-arrowBox {
    height: 100%;
    width: 1em;
    margin-right: 0.25em;
}
.dropdownMenu-arrow {
    width: 1em;
    stroke-width: 0.5em;
    transition: all 300ms ease;
}
.dropdownMenu-arrowBox.open > .dropdownMenu-arrow { transform: rotate(-90deg); }

.dropdownMenu-folder {
    position: absolute;
    top: 100%;
    z-index: 1;
    text-align: center;
    font-size: 80%;
    box-shadow: 0 0.25em 0.5em 0 rgba(0,0,0,0.2);
    min-width: 100%;
    overflow-y: hidden;
    transition: all 300ms ease;
}
.dropdownMenu-folder.closed { transform: scaleY(0); }
.dropdownMenu-folder > * {
    display: block;
    line-height: 1.5em;
    min-width: 100%;
}
</style>

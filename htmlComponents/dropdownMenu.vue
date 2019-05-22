<template>
    <div class="dropdown-menu theme-dark3"
        v-on="{ mousedown: fold, mouseenter: open, mouseleave: close }">
        <div class="dropdown-bar">
            <a v-bind:href="href">{{title}}</a>
            <svg class="dropdown-arrow"
                v-bind:class="{ open: isOpen }"
                xmlns="http://www.w3.org/2000/svg"
                viewBox="0 0 100 100">
                <path d="M 30,40  l 20,20  l 20,-20" />
            </svg>
        </div>
        <div class="dropdown-folder theme-dark3"
            @mouseenter.capture="highlight" @mouseleave.capture="unhighlight"
            v-bind:class="{ 'd-none': !isOpen }">
            <slot></slot>
        </div>
    </div>
</template>

<script>
export default {
    data() {
        return {
            isOpen: false,
        }
    },
    methods: {
        fold() { this.isOpen = !this.isOpen; },
        open() { this.isOpen = true; },
        close() { this.isOpen = false; },

        highlight(event) {
            if (!event.target.classList.contains("dropdown-folder")) {
                event.target.classList.add("theme-primary");
            }
        },
        unhighlight(event) {
            if (!event.target.classList.contains("dropdown-folder")) {
                event.target.classList.remove("theme-primary");
            }
        },
    },
    props: {
        title: { type: String, required: true },
        href: { type: String, required: true },
    },
}
</script>

<style>
.dropdown-menu {
    position: relative;
    display: inline-block;
}
.dropdown-bar {
    cursor: pointer;
    margin-left: 0.5em;
}
.dropdown-arrow {
    width: auto;
    margin: 0 0.25em;
    height: 1em;
    stroke: currentColor;
    stroke-width: 0.5em;
    fill: none;
}
.dropdown-arrow.open {
    transform: rotate(-90deg);
}
.dropdown-folder {
    position: absolute;
    display: block;
    z-index: 1;
    text-align: center;
    font-size: 80%;
    box-shadow: 0 0.25em 0.5em 0 rgba(0,0,0,0.2);
    min-width: 100%;
}
.dropdown-folder > * {
    display: block;
    height: 1.5em;
    line-height: 1.5em;
    width: 100%;
}
</style>

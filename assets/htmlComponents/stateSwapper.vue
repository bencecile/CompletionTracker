<template>
</template>

<script>
export default {
    props: {
        // The mapping from the state -> component
        // The component is given by the slot name
        stateMapping: { type: Object, required: true },
        // The current state to look up in the map
        currentState: { required: true },
    },
    computed: {
        // Finds the current component to be rendered
        currentComponent() {
            var slotName = this.stateMapping[this.currentState];
            if (!slotName) {
                // Use the default slot if we don't have a mapping
                slotName = "default";
            }
            return this.$slots[slotName];
        },
    },
    render(h) { return h("div", { class: "stateSwapper" }, this.currentComponent); },
}
</script>

<style>
.stateSwapper > * {
    margin-top: 2em;
}
</style>

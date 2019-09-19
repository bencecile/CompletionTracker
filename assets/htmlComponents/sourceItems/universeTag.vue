<template>
    <div>
        <div v-if="state === $states.Loading">
            {{ $t("loading") }}
        </div>
        <div v-if="state === $states.Error">
            {{ $t("error_occurred", [errorMessage]) }}
        </div>
        <div class="universeTag" v-if="state === $states.Success">
            <h2>{{ $langMapGet(foundUniverseTag.names) }}</h2>
            <div>{{ $langMapGet(foundUniverseTag.descriptions) }}</div>
        </div>
    </div>
</template>
<script>
export default {
    props: {
        // Either an ID or a UniverseTag object can be used
        id: { type: Number },
        universeTag: { type: Object, default: null },
    },
    data() {
        return {
            state: this.$states.Loading,
            errorMessage: null,
            foundUniverseTag: null,
        };
    },
    methods: {
        setErrorState(errorMessage) {
            this.errorMessage = errorMessage;
            this.state = this.$states.Error;
            this.$emit("onError", this.errorMessage);
        },
        setSuccessState(universeTag) {
            this.foundUniverseTag = universeTag;
            this.state = this.$states.Success;
            this.$emit("onSuccess", this.foundUniverseTag);
        }
    },
    created() {
        if (!this.universeTag) {
            // Fetch the universe tag with the ID from the props
            this.$api.readUniverseTags({
                ids: [this.id],
            }, (universeTags) => {
                this.setSuccessState(universeTags[0]);
            }, this.setErrorState);
        } else {
            this.setSuccessState(this.universeTag);
        }
    },
}
</script>
<style>
</style>

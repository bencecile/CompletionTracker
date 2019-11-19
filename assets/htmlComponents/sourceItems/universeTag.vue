<template>
    <div>
        <div v-if="state === $states.Loading">{{ $t("loading") }}</div>
        <div v-if="state === $states.Error">{{ displayErrorMessage }}</div>
        <div v-if="state === $states.Success">
            <h2>{{ $langMapGet(foundUniverseTag.names) }}</h2>
            <div>{{ $langMapGet(foundUniverseTag.descriptions) }}</div>
            <relatedLinks :relatedLinks="foundUniverseTag.related_links"/>
        </div>
    </div>
</template>
<script>
export default {
    props: {
        // TODO Add an edit button
        // Either an ID or a UniverseTag object can be used
        id: { type: Number, required: true },
        universeTag: { type: Object, default: null },
    },
    data() {
        return {
            state: this.$states.Loading,
            errorMessage: {
                rawMessage: null,
                translatedMessage: null,
            },
            foundUniverseTag: null,
        };
    },
    computed: {
        displayErrorMessage() {
            if (!this.errorMessage.translatedMessage) {
                return this.$t("error_occurred", [this.errorMessage.rawMessage]);
            } else {
                return this.$t(this.errorMessage.translatedMessage);
            }
        }
    },
    created() {
        if (!this.id) {
            this.setErrorState(null, "error_bad_route_params");
        } else {
            this.fetchUniverseTag();
        }
    },
    methods: {
        fetchUniverseTag() {
            if (this.universeTag) {
                if (this.universeTag.id != this.id) {
                    console.error(`Got passed a bad universe tag: ${this.universeTag.id}`);
                    // Fall through to do the actual fetch
                } else {
                    this.setSuccessState(this.universeTag);
                    return;
                }
            }

            this.$api.readUniverseTags({
                ids: [this.id],
            }, (universeTags) => {
                this.setSuccessState(universeTags[0]);
            }, (errorMessage) => {
                this.setErrorState(errorMessage, null);
            });
        },
        setErrorState(rawMessage, translatedMessage) {
            this.state = this.$states.Error;
            this.errorMessage.rawMessage = rawMessage;
            this.errorMessage.translatedMessage = translatedMessage;
            this.$emit("onError", this.displayErrorMessage);
        },
        setSuccessState(universeTag) {
            this.foundUniverseTag = universeTag;
            this.state = this.$states.Success;
            this.$emit("onSuccess", this.foundUniverseTag);
        }
    },
}
</script>
<style>
</style>

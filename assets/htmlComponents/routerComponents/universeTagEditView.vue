<template>
    <div>
        <h2>{{ $t("universe_tag_edit") }}</h2>

        <div v-if="fetchState == $states.Loading">{{ $t("loading") }}</div>
        <div v-if="fetchState == $states.Error">{{ displayFetchErrorMessage }}</div>
        <div v-if="fetchState == $states.Success">
            <universeTagEditor :universeTag="universeTag"/>

            <button @click="updateUniverseTag"
                :disabled="updateState == $states.Nothing">
                {{ $t("apply") }}
            </button>
        </div>
    </div>
</template>
<script>
export default {
    props: {
        id: { type: Number, required: true },
    },
    data() {
        return {
            universeTag: this.$navHolder,
            fetchState: this.$states.Loading,
            fetchErrorMessage: {
                rawMessage: null,
                translatedMessage: null,
            },
            updateState: this.$states.Nothing,
            updateErrorMessage: null,
        };
    },
    computed: {
        displayFetchErrorMessage() {
            if (!this.fetchErrorMessage.translatedMessage) {
                return this.$t("error_occurred", [this.fetchErrorMessage.rawMessage]);
            } else {
                return this.$t(this.fetchErrorMessage.translatedMessage);
            }
        },
    },
    created() {
        this.$setDocumentTitle(this.$t("universe_tag_edit"));

        if (!this.id) {
            this.setErrorState(null, "error_bad_route_params")
        } else {
            this.fetchUniverseTag();
        }
    },
    methods: {
        fetchUniverseTag() {
            if (this.universeTag) {
                if (this.universeTag.id != this.id) {
                    console.error(`Got passed the wrong universe tag: ${this.universeTag.id}`);
                    // Fall through to make the call
                } else {
                    this.state = this.$states.Success;
                    return;
                }
            }

            this.$api.readUniverseTag({ ids: [this.id] }, (universeTags) => {
                this.setFetchSuccessState(universeTags[0]);
            }, (errorMessage) => {
                this.setFetchErrorState(errorMessage, null);
            });
        },
        setFetchErrorState(rawMessage, translatedMessage) {
            this.fetchState = this.$states.Error;
            this.fetchErrorMessage.rawMessage = rawMessage;
            this.fetchErrorMessage.translatedMessage = translatedMessage;
        },
        setFetchSuccessState(universeTag) {
            this.fetchState = this.$states.Success;
            this.universeTag = universeTag;
        },

        updateUniverseTag() {
            this.state = this.$states.Loading;
        },
    },
}
</script>
<style>
</style>

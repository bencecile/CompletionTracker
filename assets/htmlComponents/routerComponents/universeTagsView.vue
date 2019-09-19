<template>
    <div>
        <router-link to="/universeTags/new">{{ $t("universe_tag_new_create") }}</router-link>
        <div v-if="state === $states.Loading">
            {{ $t("loading") }}
        </div>
        <div v-if="state === $states.Error">
            {{ $t("error_occurred", [errorMessage]) }}
        </div>
        <div class="rootUniverseTags" v-if="state === $states.Success">
            <h1>{{ $t("universe_tags") }}</h1>
            <div v-for="universeTag in rootUniverseTags" :key="universeTag.id">
                <router-link :to="'/universeTag/' + universeTag.id">
                    {{ $langMapGet(universeTag.names) }}
                </router-link>
            </div>
        </div>
    </div>
</template>
<script>
export default {
    data() {
        return {
            state: this.$states.Loading,
            errorMessage: null,
            rootUniverseTags: [],
        };
    },
    created() {
        this.$setDocumentTitle(this.$t("universe_tags"));
        this.fetchRootIds();
    },
    methods: {
        fetchRootIds() {
            this.$api.readRootUniverseTags((ids) => {
                this.fetchUniverseTags(ids);
            }, this.setErrorState);
        },
        fetchUniverseTags(ids) {
            this.$api.readUniverseTags({ ids }, this.setSuccessState, this.setErrorState);
        },

        setErrorState(errorMessage) {
            this.errorMessage = errorMessage;
            this.state = this.$states.Error;
        },
        setSuccessState(universeTags) {
            this.rootUniverseTags = universeTags;
            this.state = this.$states.Success;
        },
    },
}
</script>
<style>
</style>

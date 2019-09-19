<template>
    <div>
        <universeTag :id="id" :universeTag="universeTag"
            @onError="onError" @onSuccess="onSuccess" />
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
        }
    },
    created() {
        this.$setDocumentTitle(this.$t("universe_tag"));
    },
    methods: {
        onSuccess(universeTag) {
            this.$setDocumentTitle(this.$langMapGet(universeTag.names));
        },
        onError(errorMessage) {
        },
    },
}
</script>
<style>
</style>

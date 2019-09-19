<template>
    <div>
        This is the sources view.
        Probably show the latest added ones first.
    </div>
</template>
<script>
export default {
    created() {
        this.$setDocumentTitle(this.$t("sources"));
    }
}
</script>
<style>
</style>

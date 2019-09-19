<template>
    <div id="homeView">
        There should be some quick-look stats and navigation links here.
    </div>
</template>
<script>
export default {
    created() {
        this.$setDocumentTitle(this.$t("home"));
    }
}
</script>
<style>
</style>

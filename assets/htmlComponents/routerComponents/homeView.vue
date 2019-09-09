<template>
    <div id="homeView">
        There should be some quick-look stats and navigation links here.
    </div>
</template>
<script>
export default {
    created() {
        document.title = this.$t("completion_tracker");
    }
}
</script>
<style>
</style>

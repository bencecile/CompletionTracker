<template>
    <div>
        <router-link to="/universe_tags/new">Create a new Universe Tag</router-link>
    </div>
</template>
<script>
export default {
    created() {
        document.title = this.$t("universe_tags") + " | " + this.$t("completion_tracker");
    },
}
</script>
<style>
</style>

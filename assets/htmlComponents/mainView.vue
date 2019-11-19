<template>
    <div id="bodyContainer">
        <div id="header">
            <router-link to="/">{{ $t("home") }}</router-link>
            <router-link to="/sources">{{ $t("sources") }}</router-link>
            <router-link to="/universeTags">{{ $t("universe_tags") }}</router-link>
            <contentLangPicker/>
        </div>
        <div id="main">
            <router-view></router-view>
        </div>
    </div>
</template>
<script>
export default {
    data() {
        // TODO We may want to highlight the link where we are currently
        return {
        };
    },
}
</script>
<style>
html, body {
    width: 100%;
    max-width: 100%;
    height: 100%;
    margin: 0;
    padding: 0;
    font-family: sans-serif;
}
#bodyContainer {
    min-height: 100%;
    display: grid;
    grid-template-columns: 1fr;
    grid-template-rows: min-content 1fr;
    justify-items: center;

    row-gap: 1em;
}
#header {
    width: calc(100% - 1em);
    padding: 0.5em;
    border-bottom: 2px solid black;
}
#main {
    max-width: calc(100% - 1em);
    padding-left: 0.5em;
    padding-right: 0.5em;
}
</style>

<template>
    <div id="mainContainer">
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
    font-family: serif;
}
#mainContainer {
    min-height: 100%;
    display: grid;
    grid-template-columns: 1fr;
    grid-template-rows: min-content 1fr;
    place-items: start center;

    row-gap: 2em;
}
#header {
    padding: 0.5em;
    width: calc(100% - 1em);
    border-bottom: 2px solid black;
}
</style>

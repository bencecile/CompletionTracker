<template>
    <nav id="navHeader" class="theme-dark2">
        <!-- The link to home -->
        <a v-bind:class="{ 'theme-light1': isHomeActive }"
            class="nav-item" href="/">Home</a>
        <div id="navHeader-collapse">
            <dropdownMenu v-bind:class="{ 'theme-light1': isSourcesActive,
                'theme-light2': isSourcesSecondaryActive }"
                class="nav-item" title="Sources" href="/sources">
                <a v-bind:class="{ 'theme-light1': isUniversesActive }"
                    href="/sources/universes">Universes</a>
                <a href="#">People</a>
                <a href="#">Characters</a>
                <a href="#">Companies</a>
            </dropdownMenu>
            <!-- The link to the currently selected user's stats -->
            <dropdownMenu class="nav-item" title="My Completion" href="#">
                <a href="#">Trackers</a>
                <a href="#">Stats</a>
                <a href="#">Wishlist</a>
            </dropdownMenu>
            <!-- A dropdown menu for other application management things -->
            <dropdownMenu class="nav-item" title="Management" href="#">
                <a href="#">Settings</a>
                <button class="btn fake" v-on:click="sendShutdown">Shut down</button>
            </dropdownMenu>
        </div>
        <!-- <button class="navbar-toggler" type="button" data-toggle="collapse" data-target="#navbarSupportedContent" aria-controls="navbarSupportedContent" aria-expanded="false" aria-label="Toggle navigation">
            <span class="navbar-toggler-icon"></span>
        </button> -->
    </nav>
</template>

<script>
export default {
    computed: {
        // Here are some utility things to check for which header link is active
        isHomeActive() { return this.active === "home"; },
        isSourcesActive() { return this.active === "sources"; },
        // This secondary one is for any sub-links that are currently navigated to
        isSourcesSecondaryActive() {
            return this.isUniversesActive;
        },
        isUniversesActive() { return this.active === "universes"; },
    },
    methods: {
        sendShutdown() {
            // Send the shutdown request to the server
            simpleFetch.post("/shutdown", {}, (successText) => {
                alert(successText);
            }, (failureText) => {
                alert(failureText);
            });
        }
    },
    props: {
        active: { type: String, required: true },
    },
    // Catch the mounted life-cycle to check the width to use the mobile version
    mounted() {

    },
}
</script>

<style>
    #navHeader {
        display: flex;
        justify-content: left;
        font-size: 150%;
        height: 2em;
        padding-left: 1em;
        line-height: 2em;
    }
    #navHeader .nav-item {
        margin-right: 1em;
    }
    #navHeader a.nav-item {
        padding: 0 0.5em;
    }
</style>

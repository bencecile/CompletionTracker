<template>
    <nav id="navHeader" class="theme-dark2"
        :class="{ closed: mobileClosed }">
        <!-- The link to home -->
        <focusableA class="navHeader-home" href="/" :class="{ 'theme-light1': isHomeActive }">
            {{Global.uiStrings.home.current()}}
        </focusableA>
        <myButton class="navHeader-collapseBtn"
            @click="toggleMobileHeader">
            <svg viewBox="0 0 100 100">
                <!-- Create a menu looking thing -->
                <rect x="20" y="20" width="60" height="10"/>
                <rect x="20" y="40" width="60" height="10"/>
                <rect x="20" y="60" width="60" height="10"/>
            </svg>
        </myButton>
        <dropdownMenu :title="Global.uiStrings.sources.current()" href="/sources"
            :class="{ 'theme-light4': isSourcesActive, 'theme-light3': isSourcesRelActive }">
            <focusableA :class="{ 'theme-light4': isUniversesActive }" href="/universes">
                {{Global.uiStrings.universeTags.current()}}
            </focusableA>
            <focusableA href="#">{{Global.uiStrings.people.current()}}</focusableA>
            <focusableA href="#">{{Global.uiStrings.characters.current()}}</focusableA>
            <focusableA href="#">{{Global.uiStrings.companies.current()}}</focusableA>
        </dropdownMenu>
        <!-- The link to the currently selected user's stats -->
        <dropdownMenu :title="Global.uiStrings.myCompletion.current()" href="#">
            <focusableA href="#">{{Global.uiStrings.stats.current()}}</focusableA>
            <focusableA href="#">{{Global.uiStrings.wishlist.current()}}</focusableA>
        </dropdownMenu>
        <!-- A dropdown menu for other application management things -->
        <dropdownMenu :title="Global.uiStrings.management.current()" href="#">
            <focusableA href="#">{{Global.uiStrings.settings.current()}}</focusableA>
            <focusableA href="#">
                <myButton theme="theme-seeThrough" @click="sendShutdown">
                    {{Global.uiStrings.shutDown.current()}}
                </myButton>
            </focusableA>
        </dropdownMenu>
    </nav>
</template>

<script>
export default {
    props: {
        active: { type: String, required: true },
    },
    data() {
        return {
            // The mobile tab for all of the other navigation is closed
            mobileClosed: true,
        };
    },
    computed: {
        // Here are some utility things to check for which header link is active
        isHomeActive() { return this.active === "home"; },
        isSourcesActive() { return this.active === "sources"; },
        // Checks for any related links to the sources
        isSourcesRelActive() {
            return this.isUniversesActive;
        },
        isUniversesActive() { return this.active === "universes"; },
    },
    methods: {
        sendShutdown() {
            console.log("Shutdown");
            // Send the shutdown request to the server
            SimpleFetch.post("/shutdown", {}, (successJson) => {
                alert(successJson.data);
            }, (failureJson) => {
                alert(failureJson.errorMessage);
            });
        },
        // Toggles the mobile header closed status
        toggleMobileHeader() { this.mobileClosed = !this.mobileClosed; },
    },
}
</script>

<style>
#navHeader {
    justify-self: stretch;

    display: grid;
    font-size: 150%;
    line-height: 2em;
    padding: 0 1em;
}
#navHeader a {
    text-decoration: none;
}
.navHeader-home {
    padding: 0 0.25em;
}

@media(max-width: 767.98px) {
    #navHeader {
        overflow: visible;

        grid-template-columns: 2fr 1fr;
        grid-template-rows: 1fr;
        row-gap: 0.5em;
        column-gap: 2em;
        grid-auto-rows: max-content;
        justify-items: start;
        
        transition: all 300ms ease;
    }
    #navHeader.closed {
        overflow: hidden;
        grid-auto-rows: 0;
        row-gap: 0;
    }
    #navHeader > .dropdownMenu {
        grid-column: 1 / 3;
    }
    .navHeader-collapseBtn {
        width: 2em;
        grid-column-start: span 1;
        place-self: center center;
    }
}
@media(min-width: 767.99px) {
    #navHeader {
        justify-content: start;
        grid-auto-flow: column;
        column-gap: 0.5em;
    }
    .navHeader-collapseBtn {
        display: none;
    }
}
</style>

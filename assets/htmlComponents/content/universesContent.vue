<template>
    <div class="universesContent">
        <!-- Be able to create more universes from this screen -->
        <a href="/sources/universe/new">
            <button class="btn theme-primary">{{langStrings.universeCreate}}</button>
        </a>
        <!-- Show all of the universe that are currently here with a filter -->
    </div>
</template>

<script>
export default {
    props: {
        langStrings: {
            universeCreate: { type: String, required: true },
        },
    },
}
</script>

<style>
.universesContent {
    width: auto;
    margin: 2% 10%;
    font-size: 120%;
    text-align: center;
}
</style>

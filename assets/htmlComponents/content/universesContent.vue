<template>
    <div class="universesContent">
        <!-- Be able to go to the page to create more universes -->
        <focusableA href="/universe/new">{{Global.uiStrings.universeTagCreate.current()}}</focusableA>
        <!-- Show all of the universes that are currently here with a filter -->
    </div>
</template>

<script>
export default {
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

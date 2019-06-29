<template>
    <div class="universesNewContent theme-dark3">
        <h2>{{langStrings.universeNew}}</h2>
        <!-- We need a language selection picker -->
        <!-- TODO Use grid for the form layout -->
        <div class="form-group">
            <label for="lang">{{langStrings.language}}</label>
            <formSelect :options="langOptions" id="lang" v-model="langValue"
                :defaultSelection="thisLang"/>
        </div>
        
        <!-- We will need the name of the universe -->
        <div class="form-group">
            <label for="name">{{langStrings.name}}</label>
            <input type="text" class="theme-light1" id="name">
        </div>

        <!-- Be able to create a list of links -->

        <button class="btn theme-primary">{{langStrings.create}}</button>
    </div>
</template>

<script>
export default {
    data() {
        return {
            langValue: "",
        };
    },
    props: {
        langStrings: {
            create: { type: String, required: true },
            language: { type: String, required: true },
            name: { type: String, required: true },
            universeNew: { type: String, required: true },
        },
        langOptions: { type: Array, required: true },
        thisLang: { type: String, required: true },
    },
}
</script>

<style>
.universesNewContent {
    width: 50%;
    margin: 2ex auto;
    font-size: 130%;
    padding: 0 1em;
    padding-bottom: 1em;
}
.universesNewContent > h2 {
    text-align: center;
}
</style>

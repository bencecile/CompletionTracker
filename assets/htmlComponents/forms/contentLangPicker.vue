<template>
    <div>
        <label for="contentLangPicker">
            {{ $t("current_content_lang", [$t(value)]) }}
        </label>
        <select id="contentLangPicker"
            :value="value" @change="$emit('input', $event.target.value)">
            <option v-for="lang in allContentLangs()" :key="lang"
                :value="lang">
                {{ $t(lang) }}
            </option>
        </select>
    </div>
</template>
<script>
export default {
    props: {
        // Must be a current content lang
        value: {
            type: String,
            required: true,
            validator(value) {
                return value in CompletionTracker.contentLang.all();
            },
        },
    },
    methods: {
        allContentLangs() { return CompletionTracker.contentLang.all() },
    }
}
</script>
<style>
</style>

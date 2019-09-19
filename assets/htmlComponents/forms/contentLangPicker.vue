<template>
    <div>
        <label for="contentLangPicker">
            {{ $t("current_content_lang") }}
        </label>
        <select id="contentLangPicker"
            :value="$contentLang" @change="$contentLang = $event.target.value">
            <option v-for="lang in allContentLangs()" :key="lang"
                :value="lang">
                {{ $t(lang) }}
            </option>
        </select>
    </div>
</template>
<script>
export default {
    methods: {
        allContentLangs() { return CompletionTrackerContentLang.all() },
    }
}
</script>
<style>
</style>

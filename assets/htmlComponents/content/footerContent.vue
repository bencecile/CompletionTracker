<template>
    <div class="footer theme-dark2">
        <div>{{Global.uiStrings.runningVersion.current()}}: {{version}}</div>
        <focusableA class="footer-github"
            href="https://github.com/bencecile/CompletionTracker" target="_blank">
            <img src="/img/GitHub-Mark-Light-32px.png">
            <div>{{Global.uiStrings.githubFooter.current()}}</div>
        </focusableA>
    </div>
</template>

<script>
export default {
    props: {
        // The compiled version of the app
        version: { type: String, required: true },
    },
    data() {
        return {
        };
    },
}
</script>

<style>
.footer {
    display: grid;
    min-height: 3em;

    border-radius: 1em 1em 0 0;

    justify-content: center;

    grid-auto-flow: column;
}
.footer-github {
    display: flex;
    align-items: center;
}
.footer-github > img {
    height: 1.5em;
    margin-right: 0.5em;
}

@media(max-width: 767.98px) {
    .footer {
        width: 100%;
        grid-template-columns: max-content max-content;
        /* place-items: center space-around; */
        align-items: center;
        column-gap: 1.5em;
    }
}
@media(min-width: 767.99px) {
    .footer {
        width: 90%;
        grid-template-columns: repeat(2, 1fr);
        place-items: center center;
        column-gap: 5em;
    }
}
</style>

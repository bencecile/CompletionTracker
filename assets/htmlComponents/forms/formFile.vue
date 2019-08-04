<template>
    <div class="formFile theme-light2"
        @keydown.enter.stop @keyup.enter.stop>
        <input class="formFile-input" type="file" multiple="false"
            value=""
            :id="id" @change="selectFile">
        <myButton class="formFile-button"
            :showFocus="true" theme="theme-light2"
            @click="openFileSelect">
            {{Global.uiStrings.fileChoose.current()}}
        </myButton>
        <div class="formFile-divider"></div>
        <div class="formFile-fileName">{{selectedFileName}}</div>
    </div>
</template>

<script>
export default {
    model: {
        prop: "unusedFile",
        event: "fileSelect",
    },
    props: {
        // The ID of the input
        id: { type: String, required: true },
        // The unused file from the v-model prop
        unusedFile: { required: true },
    },
    data() {
        return {
            // The name of the selected file
            fileName: "",
        };
    },
    computed: {
        // Gets the selected file name or a filler string
        selectedFileName() {
            if (!this.fileName) {
                return Global.uiStrings.fileNotSelected.current();
            } else {
                // See if the file name will fit in the space
                const { clientWidth } = this.fileNameElement;
                const totalWidth = this.stringMeasurer.measureString(this.fileName);
                if (totalWidth > clientWidth) {
                    // The file name can't fit in the space, ellipse it in the middle
                    const extDot = this.fileName.lastIndexOf(".");
                    const fileStem = this.fileName.slice(0, extDot);
                    // Get the extension with the other ellipsis that we will show
                    const fileExt = "…" + this.fileName.slice(extDot + 1);

                    const extensionWidth = this.stringMeasurer.measureString(fileExt);
                    // Find out how many characters we can use from the file stem
                    // Use a binary search like thing to find the optimal spot
                    // The binary search needs both the start and end indices
                    //  as well as the width of the file stem up to that respective index.
                    const binarySearch = (startIndex, endIndex) => {
                        // Find the midpoint
                        const midIndex = Math.floor((endIndex + startIndex) / 2);
                        // Slice doesn't read the char at the end index
                        const midSlice = fileStem.slice(0, midIndex + 1)
                        // Find the width up to and at the midpoint
                        const midWidth = this.stringMeasurer.measureString(midSlice);

                        // Return the slice up to the middle if it fits
                        if (midWidth + extensionWidth < clientWidth) {
                            // Only return if we can't go up any more
                            if (midIndex + 1 === endIndex) {
                                return midSlice;
                            } else {
                                // Do it again but on the upper half to try to find a better fit
                                return binarySearch(midIndex, endIndex);
                            }
                        } else {
                            // If this was the last one that we could do but it doesn't fit,
                            //  we have to use the string with lower index
                            if (midIndex - 1 === startIndex) {
                                return fileStem.slice(0, startIndex + 1);
                            } else {
                                // Do it again looking for one that fits
                                return binarySearch(startIndex, midIndex);
                            }
                        }
                    };
                    const stringThatFits = binarySearch(0, extDot - 1);
                    return stringThatFits + fileExt;
                } else {
                    return this.fileName;
                }
            }
        },
    },
    methods: {
        openFileSelect() {
            // Send a click to the input
            this.$el.querySelector(`#${this.id}`).click();
        },
        // Send the event that the value of the 
        selectFile(event) {
            this.fileName = event.target.files[0].name;
            this.$emit("fileSelect", event.target.files[0]);
        },
    },
    mounted() {
        this.fileNameElement = this.$el.querySelector(".formFile-fileName");
        // Create the object that measures strings
        this.stringMeasurer = createStringMeasurer(this.fileNameElement);
    },
}
</script>

<style>
.formFile {
    display: grid;
    width: 100%;
    grid-template: 1fr / max-content min-content auto;
    place-items: center start;
    column-gap: 0.5em;
}
.formFile-input {
    position: absolute;
    visibility: hidden;
}
.formFile-divider {
    height: 100%;
    border-left: 1px solid currentColor;
}
.formFile-fileName {
    justify-self: stretch;
}
</style>

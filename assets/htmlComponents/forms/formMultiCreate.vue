<template>
    <div :id="id" class="formMultiCreate"
        :class="{ full: !singleField }"
        @keydown.enter.stop @keyup.enter.stop>
        <div v-if="!singleField" class="formMultiCreate-title">{{title}}</div>
        <template v-if="!singleField && items.length >= 1">
            <div v-for="field in fields" :key="'header' + field.id"
                class="formMultiCreate-header">
                {{field.name}}
            </div>
        </template>
        <template v-for="(item, i) in items">
            <div v-for="(field, fieldIndex) in fields" :key="field.id + i"
                class="formMultiCreate-field"
                :class="{ 'formMultiCreate-firstItem': (i === 0 && fieldIndex === 0) }">
                <label v-if="!singleField" class="formMultiCreate-label text-line"
                    :for="field.id + i">
                    {{field.name}}
                </label>
                <slot :name="field.id"
                    :id="field.id + i"
                    :modelGetValue="singleField ? item : item[field.id]"
                    :modelSetEvent="(event) => updateField(i, field.id, event)"/>
            </div>
            <myButton class="formMultiCreate-delete" theme="theme-delete"
                :key="'delete' + i"
                :showFocus="true"
                @click="deleteItem(i)">
                {{Global.uiStrings.delete.current()}}
            </myButton>
        </template>
        <myButton class="formMultiCreate-button" theme="theme-primary"
            :showFocus="true"
            @click="addItem">
            {{createText}}
        </myButton>
    </div>
</template>

<script>
export default {
    model: {
        prop: "items",
        event: "updateItems",
    },
    props: {
        // The list of items to show
        items: { type: Array, required: true },
        // The title to show for the items
        title: { type: String, default: "" },
        // The text to show for creating a new item
        createText: { type: String, required: true },
        // The data fields to use for the items
        /*{
            // The name to use on the label
            name: String,
            // The id to use for the field (as a literal ID in an item object)
            // Also used for the slot name
            id: String,
        }*/
        fields: { type: Array, required: true },
        // If the multi create will be just a single field
        singleField: { type: Boolean, default: false },
        // The ID to use to identify this multi create
        id: { type: String, required: true },
    },
    data() { return {}; },
    methods: {
        // Updates the items from the field data and maybe add a new item
        // Emit a new event to update the list of items
        updateItems(newItems) { this.$emit("updateItems", newItems); },
        // Updates the single field on the item
        updateField(itemIndex, fieldName, newValue) {
            const newItems = this.copyItems();
            if (this.singleField) {
                newItems[itemIndex] = newValue;
            } else {
                newItems[itemIndex][fieldName] = newValue;
            }
            this.updateItems(newItems);
        },
        addItem() {
            // Create the items with one added
            const newItems = this.copyItems();
            // Look through the fields to create a new object
            newItems.push(this.createNewItem());
            this.updateItems(newItems);
        },
        // Deletes the item at the index
        deleteItem(itemIndex) {
            const newItems = this.copyItems().filter((_item, i) => i !== itemIndex);
            this.updateItems(newItems);
        },

        // Creates a new item from the field descriptions
        createNewItem() {
            // Only use a string if there's only a single field
            if (this.singleField) {
                return "";
            }

            const newItem = {};
            this.fields.forEach((field) => {
                newItem[field.id] = "";
            });
            return newItem;
        },
        // Copies the items into a new array
        copyItems() {
            // We don't have an object inside to copy if it's only a single field
            if (this.singleField) {
                return Array.from(this.items);
            }

            return this.items.map((item) => Object.assign({}, item));
        }
    },
    created() {
        // Modify the grid to fit with the required data
        const styleNode = document.createElement("style");
        // We don't need a row for the title or headers if it's a single field
        const rowsTemplate = this.singleField ? "max-content 1fr" : "auto";
        const smallDeleteCol = this.singleField ? "" : `grid-column: 1 / -1;`;
        const smallColTemplate = this.singleField ?
            "grid-template-columns: 1fr min-content;" :
            "grid-template-columns: 1fr;";
        styleNode.innerHTML = `
@media(max-width: 767.98px) {
    #${this.id}.formMultiCreate {
        ${smallColTemplate}
    }
    #${this.id} > .formMultiCreate-delete {
        ${smallDeleteCol}
    }
}
@media(min-width: 767.99px) {
    #${this.id}.formMultiCreate {
        grid-template: ${rowsTemplate} / repeat(${this.fields.length}, auto) min-content;
    }
}`;
        document.head.appendChild(styleNode);
    },
}
</script>

<style>
.formMultiCreate {
    display: grid;
    width: 100%;
    grid-auto-flow: row;
    justify-items: stretch;
    grid-auto-rows: auto;
}
.formMultiCreate-title {
    justify-self: center;
}
.formMultiCreate-button {
    border: 1px dashed currentColor;
}
.formMultiCreate-header {
    font-size: 90%;
}
.formMultiCreate-firstItem {
    grid-column: 1 / 2;
}
.formMultiCreate-title {
    grid-column: 1 / -1;
}
.formMultiCreate-delete {
    grid-column: -2 / -1;
    place-self: center center;
}
.formMultiCreate-button {
    grid-column: 1 / -1;
}

@media(max-width: 767.98px) {
    .formMultiCreate-header {
        display: none;
    }
    .formMultiCreate {
        row-gap: 0.5em;
        column-gap: 0.25em;
    }
    .formMultiCreate-field {
        grid-column: 1 / 2;
    }
}
@media(min-width: 767.99px) {
    .formMultiCreate-label {
        display: none;
    }
    .formMultiCreate {
        row-gap: 0.75em;
        column-gap: 0.5em;
    }
}
</style>

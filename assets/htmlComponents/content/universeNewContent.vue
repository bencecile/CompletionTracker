<template>
    <div class="universesNewContent theme-primary"
        @keyup.enter="create">
        <h2>{{Global.uiStrings.universeTagNew.current()}}</h2>
        <div class="form-grid">
            <!-- We need a language selection picker -->
            <label class="text-line" for="lang">{{Global.uiStrings.language.current()}}</label>
            <formSelect :options="langOptions" id="lang" v-model="langValue"
                :defaultValue="thisLang"/>

            <!-- We will need the name of the universe -->
            <label class="text-line" for="name">{{Global.uiStrings.name.current()}}</label>
            <formArea id="name" :singleLine="true" v-model="nameValue"/>

            <label class="text-line" for="alias0">{{Global.uiStrings.aliases.current()}}</label>
            <formMultiCreate id="aliaseCreate" :createText="Global.uiStrings.aliasesNew.current()"
                v-model="aliases"
                :singleField="true"
                :fields="[{ 'name': 'alias', 'id': 'alias' }]">
                <template #alias="slotProps">
                    <formArea :id="slotProps.id" :singleLine="true"
                        :value="slotProps.modelGetValue"
                        @input="slotProps.modelSetEvent"/>
                </template>
            </formMultiCreate>

            <!-- The description of the universe -->
            <label class="text-line" for="description">{{Global.uiStrings.description.current()}}</label>
            <formArea id="description" v-model="descriptionValue"/>

            <!-- An uploadable image (as a file) -->
            <label class="text-line" for="image">{{Global.uiStrings.image.current()}}</label>
            <formFile id="image" v-model="imageFile" @fileSelect="imageFile = $event"/>

            <!-- Be able to create a list of links -->
            <formMultiCreate id="linkCreate" :title="Global.uiStrings.relatedLinks.current()"
                :createText="Global.uiStrings.relatedLinkNew.current()"
                v-model="links"
                :fields="[{
                    name: 'URL',
                    id: 'url',
                }, {
                    name: Global.uiStrings.description.current(),
                    id: 'description',
                }]">
                <template #url="slotProps">
                    <formArea :id="slotProps.id" :singleLine="true"
                        :value="slotProps.modelGetValue"
                        @input="slotProps.modelSetEvent"/>
                </template>
                <template #description="slotProps">
                    <formArea :id="slotProps.id"
                        :value="slotProps.modelGetValue"
                        @input="slotProps.modelSetEvent"/>
                </template>
            </formMultiCreate>
        </div>
        <myButton theme="theme-light3" :showFocus="true" @click="create">{{Global.uiStrings.create.current()}}</myButton>
        <stateSwapper :stateMapping="resultStates" :currentState="currentState">
            <template #default/>
            <template #Sending>
                <loadingResult :text="Global.uiStrings.sendingRequest.current()"/>
            </template>
            <template #FileError>
                <errorResult :text="Global.uiStrings.fileError.current()"/>
            </template>
            <template #Created>
                <createdResult :text="Global.uiStrings.universeTagCreated.current()"
                    @undo="undoCreate"
                    :link="createdLink" :linkText="Global.uiStrings.universeTagCreatedView.current()"/>
            </template>

            <template #UndoSuccess>
                <successResult :text="Global.uiStrings.undoSuccess.current()"/>
            </template>
            <template #UndoError>
                <errorResult :text="Global.uiStrings.undoError.current()"/>
            </template>
        </stateSwapper>
    </div>
</template>

<script>
export default {
    props: {
        langOptions: { type: Array, required: true },
        thisLang: { type: String, required: true },
    },
    data() {
        return {
            langValue: "",
            nameValue: "",
            aliases: [],
            descriptionValue: "",
            imageFile: null,
            links: [],

            // Make sure we can only send the creation once
            sentCreate: false,
            // Keep the ID of the created Universe
            createdId: null,
            // All of the states that have a specific component
            resultStates: {
                [ResultStates.Sending]: "Sending",
                [ResultStates.FileError]: "FileError",
                [ResultStates.Created]: "Created",

                [ResultStates.UndoError]: "UndoError",
                [ResultStates.UndoSuccess]: "UndoSuccess",
            },
            // The result state we are currently in
            currentState: ResultStates.Nothing,
        };
    },
    computed: {
        // Look up the current state in the map
        currentStateComponent() { return this.resultStates[this.currentState]; },
        // The link to the created universe
        createdLink() { return `/universe/${this.createdId}`; }
    },
    methods: {
        async create() {
            // Return first so that we can't accidentally create more than 1 entry
            if (this.sentCreate) { return; }
            this.sentCreate = true;

            this.currentState = ResultStates.Sending;

            // See if we have an image we need to read
            var image = null;
            if (this.imageFile) {
                try {
                    // Turn the file into base 64 and wait for it
                    image = await readFileBase64Promise(this.imageFile);
                } catch(e) {
                    // Failed to read the file. Display an error.
                    console.error(e);
                    this.currentState = ResultStates.FileError;
                    return;
                }
            }

            // The create data structure
            const createData = {
                lang: this.langValue,
                name: this.nameValue,
                aliases: this.aliases,
                image,
                description: this.descriptionValue,
                related_links: this.links,
            };
            console.log(createData);
            

            // After the universe has been created successfully
            this.currentState = ResultStates.Created;
        },
        // Undos the universe creation
        async undoCreate() {
            // Check for a valid universe ID first
            // There is a bug or user tampering if this happens
            if (!this.createdId) { console.error("No creation ID to undo"); return; }

            this.currentState = ResultStates.Sending;

            // Make the request to delete the created universe
        },
    },
}
</script>

<style>
.universesNewContent {
    font-size: 130%;
    padding: 1em 1em;
}
.universesNewContent > h2 {
    text-align: center;
}
@media(max-width: 767.98px) {
    .universesNewContent {
        width: 80%;
    }
}
@media(min-width: 767.99px) {
    .universesNewContent {
        width: 50%;
    }
}
</style>

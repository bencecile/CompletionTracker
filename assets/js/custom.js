// A simple map of the background colour ordering
// Looking up a theme will give you the next lighter one
const BACKGROUND_MAP = {
    "theme-dark1": "theme-dark2",
    "theme-dark2": "theme-dark3",
    "theme-dark3": "theme-primary",
    "theme-primary": "theme-light1",
    "theme-light1": "theme-light2",
    "theme-light2": "theme-light3",
    "theme-light3": "theme-light4",
    "theme-light4": "theme-light5",

    // The delete button hues
    "theme-delete": "theme-delete-hover",
    "theme-delete-hover": "theme-delete-pressed",

    // If something is supposed to be see through, it should always return the same theme
    "theme-seeThrough": "theme-seeThrough",
};

// This class will keep track of a timeout so it won't trigger if we try to set it again
class TimeoutTracker {
    // Take the function that we will want to call after the timeout
    constructor(fn) {
        // The ID of the timeout that is currently waiting to happen
        this.timeoutID = null;
        this.fn = fn;
    }
    // Sets a timeout for the number of milliseconds and passes the given args to the fn
    setTimeout(millis, ...args) {
        // Clear the old timeout
        if (this.timeoutID != null) {
            clearTimeout(this.timeoutID);
        }

        this.timeoutID = setTimeout(() => {
            // Clear the timeout ID once this has triggered
            this.timeoutID = null;
            // Call the function now
            this.fn(...args);
        }, millis);
    }
}

// The different result states that can happen. For easy lookup and less likely to fail
const ResultStates = {
    // Nothing is happening
    Nothing: Symbol("Nothing"),

    // ---- Loading States ----
    // A request is currently being sent
    Sending: Symbol("Sending"),

    // ---- Error States ----
    // The file failed to read
    FileError: Symbol("FileError"),
    // An undo failed
    UndoError: Symbol("UndoError"),

    // ---- Success States ----
    // The undo succeeded
    UndoSuccess: Symbol("UndoSuccess"),

    // ---- Creation States ----
    // Something has been created
    Created: Symbol("Created"),
};

// This is an object to simplify the fetch procedures
const SimpleFetch = {
    post(url, jsonObject, onSuccess, onFail) {
        const request = new Request(url, {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(jsonObject),
            // Make sure that we can accept and send cookies to the main server
            credentials: "same-origin",
        });
        // Converts an error into our expected JSON format
        function convertError(error) {
            return {
                success: false,
                // Make sure the error is a String
                errorMessage: new String(error),
            };
        }

        fetch(request).then(function(response) {
            // Try to read the response text body
            response.json().then(function(jsonResponse) {
                if (response.ok) {
                    onSuccess(jsonResponse);
                } else {
                    onFail(jsonResponse);
                }
            }).catch(function(error) {
                // Handle the text and body read error
                onFail(convertError(error));
            });
        }).catch(function(error) {
            // Handle the request failure
            onFail(convertError(error));
        });
    },
};

// A factory to create a canvas that can measure width of strings
// Uses the given element to match the font
function createStringMeasurer(element) {
    // Create the canvas and get the context from it
    const canvas = document.createElement("canvas");
    const context = canvas.getContext("2d");
    
    // Find out the font of the text area
    const style = getComputedStyle(element);
    context.font = `${style.fontStyle} ${style.fontSize} ${style.fontFamily}`;

    return {
        canvas,
        context,
        // Measures the width of the given string
        measureString(string) {
            return this.context.measureText(string).width;
        },
    };
}

// Reads a file and returns a Promise that will resolve to an array of bytes
function readFileBase64Promise(file) {
    const fileReader = new FileReader();
    return new Promise((resolve, reject) => {
        fileReader.onload = () => {
            // We can now look at the result property for the data
            const commaIndex = fileReader.result.indexOf(",");
            // Take everything after the first comma which ends the type
            const base64 = fileReader.result.slice(commaIndex + 1);
            resolve(base64);
        };
        fileReader.onerror = () => {
            fileReader.abort();
            reject();
        };
        // Start the reading
        fileReader.readAsDataURL(file);
    });
}

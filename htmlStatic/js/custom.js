// This class will keep track of a timeout so it won't trigger if we try to set it again
var TimeoutTracker = class {
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

// This is an object to simplify the fetch procedures
const simpleFetch = {
    post(url, jsonObject, onSuccess, onFail) {
        const request = new Request(url, {
            method: "POST",
            body: JSON.stringify(jsonObject),
        });
        fetch(request).then(function(response) {
            // Try to read the response text body
            response.text().then(function(text) {
                if (response.ok) {
                    onSuccess(text);
                } else {
                    onFail(text);
                }
            }).catch(function(error) {
                // Handle the text and body read error
                onFail(error);
            });
        }).catch(function(error) {
            // Handle the request failure
            onFail(error);
        });
    },
};

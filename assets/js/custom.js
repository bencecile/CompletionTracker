// This is an object to simplify the fetch procedures
CompletionTracker.SimpleFetch = {
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

// Make a navigation holder where we can stuff things on a navigation
//  This will be good for data objects that we would otherwise have to fetch from the server
CompletionTracker.navigationHolder = {
    holder: null
};

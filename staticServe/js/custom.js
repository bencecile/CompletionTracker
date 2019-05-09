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

$(function() {
    // Only set up filters if this is a page that has them
    if (FILTER_INFO) {
        // Do each one separately since there could be more than 1 filter
        FILTER_INFO.forEach((filterInfo) => {
            // Find the base element for this filter
            const filterBase = $(document.getElementById(filterInfo.id));
            // Find all of the data rows in the table
            const rows = filterBase.children("table > tr").slice(1);
            // Create a timeout tracker for each one
            const filterTimeout = new TimeoutTracker((query) => {
                // Figure out which rows have found a match
                const rowPass = filterInfo.filters.reduce((acc, searchList) => {
                    // Get a boolean array of matches for the search criteria
                    return searchList.map((searchItem, i) =>
                        // Reduce each one filter info down to just a single boolean array
                        searchItem.includes(query) | acc[i]
                    );
                }, Array(rows.length).fill(false));
                // Hide all of the rows that didn't match and make sure we can see the good ones
                rows.filter((i, _) => !rowPass[i]).addClass("d-none");
                rows.filter((i, _) => rowPass[i]).removeClass("d-none");
            });
            // Set up the event listener for the input box
            filterBase.children("input.filter_search").on("keyup", function(event) {
                // Set the timeout and filter using the contents of the input box
                filterTimeout.setTimeout(200, event.target.value);
            });
        });
    }
});

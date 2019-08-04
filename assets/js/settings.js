// Look in the storage for the settings TODO

// Set the global settings
Global.settings = {
    currentLang: Global.langs[0],
}

// A simple function to get show a language with the current language from the uiString
Global.uiStrings.prototype.current = function() { return this[Global.settings.currentLang] };

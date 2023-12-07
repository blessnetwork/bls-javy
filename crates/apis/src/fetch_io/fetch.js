// Wrap everything in an anonymous function to avoid leaking local variables into the global scope.
(function () {
    // Get a reference to the function before we delete it from `globalThis`.
    const __javy_fetchio_get = globalThis.__javy_fetchio_get;
    globalThis.Javy.FetchIO = {
        get(url) {
            __javy_fetchio_get(url);
            },
    };
    // Delete the function from `globalThis` so it doesn't leak.
    Reflect.deleteProperty(globalThis, "__javy_fetchio_get");
})();
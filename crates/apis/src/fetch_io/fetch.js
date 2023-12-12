// Wrap everything in an anonymous function to avoid leaking local variables into the global scope.
(function () {
    // Get a reference to the function before we delete it from `globalThis`.
    const __javy_fetchio_request = globalThis.__javy_fetchio_request;

    function fetch(url, options = {}) {
        const encodedOutput = new TextEncoder().encode(JSON.stringify(options))
        const data = new Uint8Array(encodedOutput)

        const responseObj = __javy_fetchio_request(url, data.buffer, data.byteOffset, data.byteLength);

        // @TODO: Capture all response data from response object
        const responseOk = true;
        const responseHeaders = {};

        return new Promise((resolve, reject) => {
            const response = {
                url,
                headers: responseHeaders,
                ok: responseOk,
                type: typeof responseObj === 'string' ? 'text' : 'json',
                text: () => typeof responseObj === 'string' ? responseObj : JSON.stringify(responseObj),
                json: () => typeof responseObj !== 'string' ? responseObj : JSON.parse(responseObj),
            };

            resolve(response);
        });
    }

    globalThis.fetch = fetch

    // Delete the function from `globalThis` so it doesn't leak.
    Reflect.deleteProperty(globalThis, "__javy_fetchio_request");
})();
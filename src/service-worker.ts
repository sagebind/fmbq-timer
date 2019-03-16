self.addEventListener("fetch", (event: any) => {
    event.respondWith((async () => {
        const cache = await caches.open("static");
        let response = await cache.match(event.request);

        if (!response) {
            console.log("Fetching resource from network: " + event.request.url);
            response = await fetch(event.request);
            cache.put(event.request, response.clone());
        }

        return response;
    })());
});

self.addEventListener("install", (event: any) => {
    event.waitUntil((async () => {
        const cache = await caches.open("static");
        await cache.addAll([
            ".",
            "index.html",
            "app.css",
            "app.js",
            "https://unpkg.com/mithril@1.1.6/mithril.js",
            "https://unpkg.com/picnic@6.5.0/picnic.min.css",
            "https://unpkg.com/systemjs@0.21.6/dist/system.js",
            "sounds/bugaboo.wav",
            "sounds/bumptious.wav",
            "sounds/correct.wav",
            "sounds/fanta.wav",
            "sounds/foofaraw.wav",
            "sounds/ludic.wav",
            "sounds/tizzy.wav",
            "sounds/unctuous.wav",
        ]);
    })());
});

self.addEventListener("fetch", (event: any) => {
    event.respondWith((async () => {
        const cache = await caches.open("static");

        try {
            return await cache.match(event.request)
        } catch (e) {
            const response = await fetch(event.request);
            cache.put(event.request, response.clone());
            return response;
        }
    })());
});

self.addEventListener("install", function (event) {
    event.waitUntil(
        caches.open("static").then(function (cache) {
            return cache.addAll([
                ".",
                "index.html",
                "app.css",
                "app.js",
                "picnic.min.css",
                "service-worker.js",
            ]);
        })
    );
});

self.addEventListener("fetch", function (event) {
    event.respondWith(
        caches.open("static").then(function (cache) {
            return cache.match(event.request).then(function (response) {
                return response || fetch(event.request).then(function (response) {
                    cache.put(event.request, response.clone());
                    return response;
                });
            });
        })
    );
});

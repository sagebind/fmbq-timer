self.addEventListener("install", function (event) {
    event.waitUntil(
        caches.open("static").then(function (cache) {
            return cache.addAll([
                ".",
                "index.html",
                "app.css",
                "app.js",
                "https://unpkg.com/mithril@1.1.6/mithril.js",
                "vendor/picnic.min.css",
                "vendor/vue.min.js",
                "sounds/bugaboo.wav",
                "sounds/bumptious.wav",
                "sounds/correct.wav",
                "sounds/fanta.wav",
                "sounds/foofaraw.wav",
                "sounds/ludic.wav",
                "sounds/tizzy.wav",
                "sounds/unctuous.wav",
                "themes/dark.css",
                "themes/joyful.css",
                "themes/light.css",
            ]);
        })
    );
});

self.addEventListener("fetch", function (event) {
    event.respondWith(
        fetch(event.request).then(function (response) {
            return caches.open("static").then(function (cache) {
                cache.put(event.request, response.clone());
                return response;
            });
        }).catch(function () {
            return caches.match(event.request);
        })
    );
});

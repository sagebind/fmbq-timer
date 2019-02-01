var audio = new Audio();

window.app = new Vue({
    el: "main",

    data: function () {
        var settings = {
            theme: "light",
            timerSound: "tizzy",
        };

        var json = localStorage.getItem("settings");
        if (json) {
            Object.assign(settings, JSON.parse(json));
        }

        return {
            currentPage: "timer",
            installed: false,
            installPrompt: null,
            interval: 0,
            settings: settings,
            timer: {
                active: false,
                startTime: 0,
                endTime: 0,
                remaining: 0,
            },
        };
    },

    computed: {
        timerText: function () {
            return (this.timer.remaining / 1000).toFixed(1);
        },

        timerTotal: function () {
            return this.timer.endTime - this.timer.startTime;
        },

        countdownCircleStyle: function () {
            if (this.timer.active) {
                var fraction = this.timer.remaining / this.timerTotal;
                return (90 * Math.PI * fraction) + " 300";
            }

            return "300 300";
        },
    },

    watch: {
        currentPage: function (name) {
            document.querySelectorAll("main > section").forEach(function (page) {
                if (page.id === name) {
                    page.classList.add("active");
                } else {
                    page.classList.remove("active");
                }
            });

            window.scrollTo(0, 0);
        },

        // Save settings automatically on change.
        settings: {
            deep: true,
            handler: function (settings) {
                localStorage.setItem("settings", JSON.stringify(settings));
            },
        },

        "settings.theme": {
            immediate: true,
            handler: function (theme) {
                document.getElementById("theme").href = "themes/" + theme + ".css";
            },
        },

        "settings.timerSound": {
            immediate: true,
            handler: function (newValue, oldValue) {
                audio.src = "sounds/" + newValue + ".wav";
                if (oldValue) {
                    this.playTimerSound();
                }
            },
        },
    },

    methods: {
        install: function () {
            if (this.installed) {
                alert("App is already installed!");
            } else if (!this.installPrompt) {
                alert("Install is not supported on your device.");
            } else {
                this.installPrompt.prompt();
                this.installPrompt.userChoice.then(function (result) {
                    if (result.outcome === "accepted") {
                        app.installed = true;
                    }
                    app.installPrompt = null;
                });
            }
        },

        update: function () {
            if ("caches" in window) {
                caches.delete("static").then(function () {
                    window.location.reload();
                });
            } else {
                window.location.reload();
            }
        },

        startTimer: function (seconds) {
            this.timer.startTime = performance.now();
            this.timer.endTime = this.timer.startTime + (seconds * 1000);
            this.timer.active = true;
        },

        cancelTimer: function () {
            this.timer.active = false;
            this.timer.remaining = 0;
        },

        playTimerSound: function () {
            audio.currentTime = 0;
            audio.play();
        },

        tick: function () {
            if (this.timer.active) {
                var remaining = Math.max(0, this.timer.endTime - performance.now());
                this.timer.remaining = remaining;

                if (remaining <= 0) {
                    this.timer.active = false;
                    this.playTimerSound();
                }
            }
        },

        showCurrentPage: function () {
            if (location.hash.length > 1) {
                this.currentPage = location.hash.substring(1);
            } else {
                this.currentPage = "timer";
            }
        },
    },

    created: function () {
        // Set up offline cache handling.
        if ("serviceWorker" in navigator) {
            navigator.serviceWorker.register("service-worker.js");
        }

        this.showCurrentPage();

        if (window.matchMedia("(display-mode: standalone)").matches) {
            this.installed = true;
        }

        this.interval = setInterval(this.tick, 50);

        setTimeout(function () {
            document.body.classList.add("ready");
        }, 100);
    },

    beforeDestroy: function () {
        clearInterval(this.interval);
    },
});

window.addEventListener("hashchange", app.showCurrentPage);

window.addEventListener("beforeinstallprompt", function (e) {
    e.preventDefault();
    app.installPrompt = e;
});

window.addEventListener("appinstalled", function () {
    app.installed = true;
});

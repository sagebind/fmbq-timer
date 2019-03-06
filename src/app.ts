import {MainPage, SettingsPage} from './components';
import {settings} from './settings';
import {state} from './state';

declare var m: any;
declare var Promise: any;


var audio = new Audio();

window.addEventListener("beforeinstallprompt", function (e) {
    e.preventDefault();
    state.installPrompt = e;
});

window.addEventListener("appinstalled", function () {
    state.installed = true;
});

export var App = {
    saveSettings: function () {
        localStorage.setItem("settings", JSON.stringify(settings));
    },

    install: function () {
        if (state.installed) {
            alert("App is already installed!");
        } else if (!state.installPrompt) {
            alert("Install is not supported on your device.");
        } else {
            state.installPrompt.prompt();
            state.installPrompt.userChoice.then(function (result) {
                if (result.outcome === "accepted") {
                    state.installed = true;
                }
                state.installPrompt = null;
            });
        }
    },

    update: function () {
        if ("caches" in window) {
            caches.delete("static").then(function () {
                location.reload();
            });
        } else {
            location.reload();
        }
    },

    setTimerSound: function (name: string) {
        settings.timerSound = name;
        audio.src = "sounds/" + name + ".wav";
    },

    startTimer: function (seconds) {
        state.timer.startTime = performance.now();
        state.timer.endTime = state.timer.startTime + (seconds * 1000);
        state.timer.active = true;
    },

    cancelTimer: function () {
        state.timer.active = false;
        state.timer.remaining = 0;
    },

    playTimerSound: function () {
        audio.currentTime = 0;
        audio.play();
    },

    tick: function () {
        if (state.timer.active) {
            var remaining = Math.max(0, state.timer.endTime - performance.now());
            state.timer.remaining = remaining;

            if (remaining <= 0) {
                state.timer.active = false;
                App.playTimerSound();
            }

            m.redraw();
        }
    },

    init: function () {
        // Set up offline cache handling.
        if ("serviceWorker" in navigator) {
            navigator.serviceWorker.register("service-worker.js");
        }

        var json = localStorage.getItem("settings");
        if (json) {
            Object.assign(settings, JSON.parse(json));
        }

        if (window.matchMedia("(display-mode: standalone)").matches) {
            state.installed = true;
        }

        App.setTimerSound(settings.timerSound);

        setInterval(App.tick, 50);

        function page(component) {
            return {
                onmatch: function () {
                    var previousPage = document.querySelector("body > main") as HTMLElement;
                    if (previousPage) {
                        previousPage.style.opacity = "0";
                    }

                    return new Promise(function (resolve) {
                        setTimeout(function () {
                            resolve(component);
                        }, 200);
                    });
                },
            };
        }

        m.route(document.body, "/", {
            "/": page(MainPage),
            "/settings": page(SettingsPage),
        });
    },
};

window['App'] = App;
App.init();

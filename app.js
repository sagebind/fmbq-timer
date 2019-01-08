var availableSounds = {
    sweetalert1: {
        name: "Sweet Alert 1",
        src: "sounds/72125__kizilsungur__sweetalertsound1.wav",
    },
    sweetalert3: {
        name: "Sweet Alert 3",
        src: "sounds/72127__kizilsungur__sweetalertsound3.wav",
    },
    sweetalert4: {
        name: "Sweet Alert 4",
        src: "sounds/72128__kizilsungur__sweetalertsound4.wav",
    },
    sweetalert5: {
        name: "Sweet Alert 5",
        src: "sounds/72129__kizilsungur__sweetalertsound5.wav",
    },
    sweetalert: {
        name: "Sweet Alert",
        src: "sounds/204369__philitup321__alert-sound.ogg",
    },
    foofaraw: {
        name: "Foofaraw",
        src: "sounds/276607__mickleness__ringtone-foofaraw.wav",
    },
    fanta: {
        name: "Fanta",
        src: "sounds/276608__mickleness__ringtone-fanta.wav",
    },
    bumptious: {
        name: "Bumptious",
        src: "sounds/276609__mickleness__notification-bumptious.wav",
    },
    bugaboo: {
        name: "Bugaboo",
        src: "sounds/276610__mickleness__notification-bugaboo.wav",
    },
    unctuous: {
        name: "Unctuous",
        src: "sounds/276611__mickleness__notification-unctuous.wav",
    },
    tizzy: {
        name: "Tizzy",
        src: "sounds/276612__mickleness__ringtone-tizzy.wav",
    },
    ludic: {
        name: "Ludic",
        src: "sounds/276614__mickleness__ludic.wav",
    },
    correct: {
        name: "Correct",
        src: "sounds/415762__thebuilder15__notification-correct.wav",
    },
}

var state = {
    active: false,
    startTime: 0,
    endTime: 0,
    audio: new Audio(),
}

window.app = {
    settings: {
        timerSound: "tizzy",
        answerDuration: 20,
        appealDuration: 30,
        timeoutDuration: 60,
    },

    loadSettings: function () {
        var json = localStorage.getItem("settings");
        if (json) {
            Object.assign(app.settings, JSON.parse(json));
        }
    },

    saveSettings: function () {
        localStorage.setItem("settings", JSON.stringify(app.settings));
    },

    showPage: function (name) {
        document.querySelectorAll("a[href^=\"#\"]").forEach(function (tab) {
            if (tab.getAttribute("href") === "#" + name) {
                tab.classList.add("active");
            } else {
                tab.classList.remove("active");
            }
        });

        document.querySelectorAll("main > section").forEach(function (page) {
            if (page.id === name) {
                page.classList.add("active");
            } else {
                page.classList.remove("active");
            }
        });
    },

    showCurrentPage: function () {
        if (location.hash.length > 1) {
            app.showPage(location.hash.substring(1));
        } else {
            app.showPage("timer");
        }
    },

    startTimer: function (seconds) {
        state.startTime = performance.now();
        state.endTime = state.startTime + (seconds * 1000);
        state.active = true;
    },

    cancelTimer: function () {
        state.active = false;
    },

    install: function () {
        if (window.matchMedia("(display-mode: standalone)").matches) {
            alert("App is already installed!");
        } else {
            alert("Install is not supported on your device.");
        }
    },

    playSound: function () {
        state.audio.src = availableSounds[app.settings.timerSound].src;
        state.audio.currentTime = 0;
        state.audio.play();
    },

    tick: function () {
        if (state.active) {
            var total = state.endTime - state.startTime;
            var remaining = Math.max(0, state.endTime - performance.now());

            app.setCountdownValue(remaining);
            app.setCircleFraction(remaining / total);

            if (remaining <= 0) {
                state.active = false;
                app.playSound();
            }
        } else {
            app.setCountdownValue(0);
            app.setCircleFraction(1);
        }
    },

    setCountdownValue: function (value) {
        var text = (value / 1000).toFixed(1);

        if (app.countdownText.textContent != text) {
            app.countdownText.textContent = text;
        }
    },

    setCircleFraction: function (fraction) {
        app.countdownCircle.setAttribute("stroke-dasharray", (90 * Math.PI * fraction) + " 1000");
    },

    initUi: function () {
        app.countdownText = document.querySelector(".countdown-display .countdown-text");
        app.countdownCircle = document.querySelector(".countdown-display .countdown-circle");

        var soundsSelectBox = document.querySelector("#sounds-select-box");

        Object.keys(availableSounds).forEach(function (id) {
            var option = document.createElement("option");
            option.value = id;
            option.innerText = availableSounds[id].name;
            soundsSelectBox.appendChild(option);
        });

        soundsSelectBox.value = app.settings.timerSound;

        soundsSelectBox.addEventListener("change", function (e) {
            app.settings.timerSound = e.target.value;
            app.playSound();
            app.saveSettings();
        });
    },
}

window.addEventListener("hashchange", app.showCurrentPage);

window.addEventListener("beforeinstallprompt", function (promptEvent) {
    promptEvent.preventDefault();
    app.install = function () {
        promptEvent.prompt();
    };
});

document.addEventListener("DOMContentLoaded", function () {
    // Make the app useable right away.
    app.loadSettings();
    app.initUi();
    app.showCurrentPage();
    window.setInterval(app.tick, 50);

    // Set up offline cache handling.
    if ("serviceWorker" in navigator) {
        navigator.serviceWorker.register("service-worker.js");
    }

    if ("caches" in window) {
        caches.open("static").then(function (cache) {
            cache.addAll(Object.keys(availableSounds).map(function (id) {
                return availableSounds[id].src;
            }));
        });
    }
});

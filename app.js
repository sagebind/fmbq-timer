var availableSounds = [
    {
        id: "bugaboo",
        name: "Bugaboo",
    },
    {
        id: "bumptious",
        name: "Bumptious",
    },
    {
        id: "correct",
        name: "Correct",
    },
    {
        id: "fanta",
        name: "Fanta",
    },
    {
        id: "foofaraw",
        name: "Foofaraw",
    },
    {
        id: "ludic",
        name: "Ludic",
    },
    {
        id: "tizzy",
        name: "Tizzy",
    },
    {
        id: "unctuous",
        name: "Unctuous",
    },
]

var state = {
    timerActive: false,
    startTime: 0,
    endTime: 0,
    audio: new Audio(),
}

function setCountdownValue(value) {
    var text = (value / 1000).toFixed(1);

    if (app.countdownText.textContent != text) {
        app.countdownText.textContent = text;
    }
}

function setCountdownCircle(fraction) {
    app.countdownCircle.setAttribute("stroke-dasharray", (90 * Math.PI * fraction) + " 1000");
}

function setTimerActive(active) {
    state.timerActive = !!active;

    if (active) {
        document.body.classList.add("timer-active");
    } else {
        document.body.classList.remove("timer-active");
    }
}

function playTimerSound() {
    state.audio.src = "sounds/" + app.settings.timerSound + ".wav";
    state.audio.currentTime = 0;
    state.audio.play();
}

window.app = {
    settings: {
        theme: "light",
        timerSound: "tizzy",
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

    setTheme: function (theme, callback) {
        app.settings.theme = theme;
        app.saveSettings();

        var link = document.getElementById("theme");
        if (callback) {
            link.addEventListener("load", callback, { once: true });
        }
        link.href = "themes/" + theme + ".css";
    },

    setTimerSound: function (name) {
        if (app.settings.timerSound !== name) {
            app.settings.timerSound = name;
            playTimerSound();
            app.saveSettings();
        }
    },

    showPage: function (name) {
        document.querySelectorAll("main > section").forEach(function (page) {
            if (page.id === name) {
                page.classList.add("active");
            } else {
                page.classList.remove("active");
            }
        });

        window.scrollTo(0, 0);
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
        setTimerActive(true);
    },

    cancelTimer: function () {
        setTimerActive(false);
    },

    install: function () {
        if (window.matchMedia("(display-mode: standalone)").matches) {
            alert("App is already installed!");
        } else {
            alert("Install is not supported on your device.");
        }
    },

    tick: function () {
        if (state.timerActive) {
            var total = state.endTime - state.startTime;
            var remaining = Math.max(0, state.endTime - performance.now());

            setCountdownValue(remaining);
            setCountdownCircle(remaining / total);

            if (remaining <= 0) {
                setTimerActive(false);
                playTimerSound();
            }
        } else {
            setCountdownValue(0);
            setCountdownCircle(1);
        }
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
    // Set up offline cache handling.
    if ("serviceWorker" in navigator) {
        navigator.serviceWorker.register("service-worker.js");
    }

    app.loadSettings();

    app.countdownText = document.querySelector(".countdown-display .countdown-text");
    app.countdownCircle = document.querySelector(".countdown-display .countdown-circle");

    document.getElementById("setting-theme").value = app.settings.theme;
    document.getElementById("setting-timer-sound").value = app.settings.timerSound;

    app.setTheme(app.settings.theme);
    app.showCurrentPage();

    setInterval(app.tick, 50);

    setTimeout(function () {
        document.body.classList.add("ready");
    }, 100);
});

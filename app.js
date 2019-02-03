var audio = new Audio();

var settings = {
    theme: "light",
    timerSound: "tizzy",
};

var state = {
    installed: false,
    installPrompt: null,
    interval: 0,
    timer: {
        active: false,
        startTime: 0,
        endTime: 0,
        remaining: 0,
    },

    timerText: function () {
        return (state.timer.remaining / 1000).toFixed(1);
    },
};

// window.app = new Vue({
//     el: "main",

//     data: function () {
//         var settings = {
//             theme: "light",
//             timerSound: "tizzy",
//         };

//         var json = localStorage.getItem("settings");
//         if (json) {
//             Object.assign(settings, JSON.parse(json));
//         }

//         return {
//             currentPage: "timer",
//             installed: false,
//             installPrompt: null,
//             interval: 0,
//             settings: settings,
//             timer: {
//                 active: false,
//                 startTime: 0,
//                 endTime: 0,
//                 remaining: 0,
//             },
//         };
//     },

//     computed: {
//         timerText: function () {
//             return (this.timer.remaining / 1000).toFixed(1);
//         },

//         timerTotal: function () {
//             return this.timer.endTime - this.timer.startTime;
//         },

//         countdownCircleStyle: function () {
//             if (this.timer.active) {
//                 var fraction = this.timer.remaining / this.timerTotal;
//                 return (90 * Math.PI * fraction) + " 300";
//             }

//             return "300 300";
//         },
//     },

//     watch: {
//         currentPage: function (name) {
//             document.querySelectorAll("main > section").forEach(function (page) {
//                 if (page.id === name) {
//                     page.classList.add("active");
//                 } else {
//                     page.classList.remove("active");
//                 }
//             });

//             window.scrollTo(0, 0);
//         },

//         // Save settings automatically on change.
//         settings: {
//             deep: true,
//             handler: function (settings) {
//                 localStorage.setItem("settings", JSON.stringify(settings));
//             },
//         },

//         "settings.theme": {
//             immediate: true,
//             handler: function (theme) {
//                 document.getElementById("theme").href = "themes/" + theme + ".css";
//             },
//         },

//         "settings.timerSound": {
//             immediate: true,
//             handler: function (newValue, oldValue) {
//                 audio.src = "sounds/" + newValue + ".wav";
//                 if (oldValue) {
//                     this.playTimerSound();
//                 }
//             },
//         },
//     },

//     methods: {
//         install: function () {
//             if (this.installed) {
//                 alert("App is already installed!");
//             } else if (!this.installPrompt) {
//                 alert("Install is not supported on your device.");
//             } else {
//                 this.installPrompt.prompt();
//                 this.installPrompt.userChoice.then(function (result) {
//                     if (result.outcome === "accepted") {
//                         app.installed = true;
//                     }
//                     app.installPrompt = null;
//                 });
//             }
//         },

//         update: function () {
//             if ("caches" in window) {
//                 caches.delete("static").then(function () {
//                     window.location.reload();
//                 });
//             } else {
//                 window.location.reload();
//             }
//         },

//         startTimer: function (seconds) {
//             this.timer.startTime = performance.now();
//             this.timer.endTime = this.timer.startTime + (seconds * 1000);
//             this.timer.active = true;
//         },

//         cancelTimer: function () {
//             this.timer.active = false;
//             this.timer.remaining = 0;
//         },

//         playTimerSound: function () {
//             audio.currentTime = 0;
//             audio.play();
//         },

//         tick: function () {
//             if (this.timer.active) {
//                 var remaining = Math.max(0, this.timer.endTime - performance.now());
//                 this.timer.remaining = remaining;

//                 if (remaining <= 0) {
//                     this.timer.active = false;
//                     this.playTimerSound();
//                 }
//             }
//         },

//         showCurrentPage: function () {
//             if (location.hash.length > 1) {
//                 this.currentPage = location.hash.substring(1);
//             } else {
//                 this.currentPage = "timer";
//             }
//         },
//     },

//     created: function () {
//         // Set up offline cache handling.
//         if ("serviceWorker" in navigator) {
//             navigator.serviceWorker.register("service-worker.js");
//         }

//         this.showCurrentPage();

//         if (window.matchMedia("(display-mode: standalone)").matches) {
//             this.installed = true;
//         }

//         this.interval = setInterval(this.tick, 50);

//         setTimeout(function () {
//             document.body.classList.add("ready");
//         }, 100);
//     },

//     beforeDestroy: function () {
//         clearInterval(this.interval);
//     },
// });

// window.addEventListener("hashchange", app.showCurrentPage);

// window.addEventListener("beforeinstallprompt", function (e) {
//     e.preventDefault();
//     app.installPrompt = e;
// });

// window.addEventListener("appinstalled", function () {
//     app.installed = true;
// });


var App = {
    saveSettings: function () {
        localStorage.setItem("settings", JSON.stringify(settings));
    },

    setTheme: function (theme) {
        settings.theme = theme;
        document.getElementById("theme").href = "themes/" + theme + ".css";
        App.saveSettings();
    },

    setTimerSound: function (name) {
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

    init: function () {
        // Set up offline cache handling.
        if ("serviceWorker" in navigator) {
            navigator.serviceWorker.register("service-worker.js");
        }

        var json = localStorage.getItem("settings");
        if (json) {
            Object.assign(settings, JSON.parse(json));
        }

        setInterval(function () {
            if (state.timer.active) {
                var remaining = Math.max(0, state.timer.endTime - performance.now());
                state.timer.remaining = remaining;

                if (remaining <= 0) {
                    state.timer.active = false;
                    App.playTimerSound();
                }

                m.redraw();
            }
        }, 50);

        function page(component) {
            return {
                onmatch: function() {
                    var previousPage = document.querySelector("body > main");
                    if (previousPage) {
                        previousPage.style.opacity = 0;
                    }

                    return new Promise(function (resolve) {
                        setTimeout(function () {
                            resolve(component);
                        }, 200);
                    });
                },
            };
        }

        m.route(document.body, "/main", {
            "/main": page(MainPage),
            "/settings": page(SettingsPage),
        });

        setTimeout(function () {
            document.body.classList.add("ready");
        }, 100);
    },
};

var MainPage = {
    view: function () {
        return [
            m("header", [
                m(".brand", "FMBQ Timer"),
                m("a.right[title=Settings]", {
                    href: "#!/settings",
                }, [
                    m("svg.icon", {
                        viewBox: "0 0 24 24"
                    }, [
                        m("path", {
                            d: "M12 8c-2.2 0-4 1.8-4 4s1.8 4 4 4c2.2 0 4-1.8 4-4s-1.8-4-4-4zM12 14c-1.1 0-2-0.9-2-2s0.9-2 2-2c1.1 0 2 0.9 2 2s-0.9 2-2 2z"
                        }),
                        m("path", {
                            d: "M20.3 15.4c0.1-0.2 0.3-0.4 0.7-0.4 1.7 0 3-1.3 3-3s-1.3-3-3-3h-0.2c-0.2 0-0.4-0.1-0.5-0.3 0-0.1 0-0.1-0.1-0.2-0.1-0.2-0.1-0.5 0.2-0.8 1.2-1.2 1.2-3.1 0-4.2v0c0 0 0 0 0 0-0.6-0.6-1.3-0.9-2.1-0.9 0 0 0 0 0 0-0.8 0-1.6 0.3-2.2 0.9-0.2 0.2-0.5 0.2-0.7 0.1-0.2 0-0.4-0.3-0.4-0.6 0-1.7-1.3-3-3-3s-3 1.3-3 3v0.2c0 0.2-0.1 0.4-0.3 0.5-0.1 0-0.1 0-0.2 0.1-0.2 0.1-0.5 0-0.8-0.2-1.2-1.2-3.1-1.2-4.2 0-1.2 1.2-1.2 3.1 0.1 4.3 0.2 0.2 0.2 0.5 0.1 0.8-0.1 0.2-0.4 0.4-0.7 0.4-1.7 0-3 1.3-3 3s1.3 3 3 3h0.2c0.3 0 0.5 0.2 0.6 0.4s0.1 0.5-0.2 0.8c-0.6 0.6-0.9 1.3-0.9 2.1s0.3 1.5 0.9 2.1c0 0 0 0 0 0 1.2 1.2 3.1 1.2 4.3-0.1 0.2-0.2 0.5-0.2 0.8-0.1s0.4 0.3 0.4 0.7c0 1.7 1.3 3 3 3s3-1.3 3-3v-0.2c0-0.3 0.2-0.5 0.4-0.6s0.5-0.1 0.8 0.2c1.2 1.2 3.1 1.2 4.2 0 1.2-1.2 1.2-3.1-0.1-4.3-0.1-0.2-0.2-0.5-0.1-0.7 0 0 0 0 0 0zM18.5 14.6c-0.4 1-0.2 2.1 0.6 3 0.2 0.2 0.3 0.4 0.3 0.7s-0.1 0.5-0.3 0.7c-0.2 0.2-0.4 0.3-0.7 0.3 0 0 0 0 0 0-0.3 0-0.5-0.1-0.8-0.4-0.8-0.8-1.9-1-2.9-0.5-1 0.4-1.6 1.4-1.6 2.4v0.2c0 0.6-0.4 1-1 1s-1-0.4-1-1c0 0 0-0.1 0-0.1 0-1.1-0.7-2-1.7-2.4-0.3-0.2-0.7-0.2-1.1-0.2-0.7 0-1.4 0.3-1.9 0.8-0.4 0.4-1 0.4-1.4 0 0 0 0 0 0 0v0c-0.2-0.2-0.3-0.4-0.3-0.7s0.1-0.5 0.4-0.8c0.8-0.8 1-1.9 0.5-2.9-0.4-1-1.4-1.6-2.4-1.6h-0.2c-0.6 0-1-0.4-1-1s0.4-1 1-1c0 0 0.1 0 0.1 0 1.1 0 2-0.7 2.4-1.7s0.2-2.1-0.6-3c-0.4-0.4-0.4-1 0-1.4s1-0.4 1.5 0.1c0.7 0.7 1.8 0.9 2.7 0.6 0.1 0 0.2 0 0.3-0.1 1-0.4 1.6-1.4 1.6-2.4v-0.2c0-0.6 0.4-1 1-1s1 0.4 1 1.1c0 1.1 0.6 2 1.6 2.4s2.1 0.2 3-0.6c0.2-0.2 0.4-0.3 0.7-0.3v0c0.3 0 0.5 0.1 0.7 0.3 0 0 0 0 0 0 0.4 0.4 0.4 1-0.1 1.5-0.7 0.7-0.9 1.8-0.6 2.7 0 0.1 0 0.2 0.1 0.3 0.4 1 1.4 1.6 2.4 1.6h0.2c0.6 0 1 0.4 1 1s-0.4 1-1.1 1c-1 0-2 0.6-2.4 1.6z"
                        }),
                    ]),
                ]),
            ]),

            m("main#timer", [
                m(".countdown-display", [
                    m("svg", {
                        viewBox: "0 0 100 100",
                        onclick: function () {
                            App.startTimer(20);
                        },
                    }, [
                        m("circle.countdown-circle", {
                            cx: 50,
                            cy: 50,
                            r: 45,
                            fill: "none",
                            stroke: "currentColor",
                            transform: "rotate(-90, 50, 50)",
                            "stroke-dasharray": "300 300",
                        }),
                        m("text.countdown-text", {
                            x: 50,
                            y: 50,
                            style: "font-size: 32px",
                            fill: "currentColor",
                            "dominant-baseline": "central",
                        }, state.timerText()),
                    ]),

                    m("button.round.reset-button.warning[title=Cancel]", {
                        class: state.timer.active ? "active" : "",
                        onclick: App.cancelTimer,
                    }, [
                        m("svg.icon[viewBox='0 0 24 24']", [
                            m("path", {
                                d: "M21.4 8.7c-0.9-2.5-2.7-4.5-5.1-5.7s-5.1-1.3-7.6-0.4c-1.4 0.5-2.7 1.3-3.7 2.3l-3 2.8v-3.7c0-0.6-0.4-1-1-1s-1 0.4-1 1v6c0 0 0 0.1 0 0.1 0 0.1 0 0.2 0 0.2 0 0.1 0.1 0.1 0.1 0.2 0 0 0 0.1 0.1 0.1 0 0 0 0 0 0 0.1 0.1 0.1 0.1 0.2 0.1 0 0 0.1 0.1 0.1 0.1s0.1 0 0.1 0 0.2 0 0.2 0c0 0 0 0 0 0h6c0.6 0 1-0.4 1-1s-0.2-0.8-0.8-0.8h-3.5l2.8-2.7c0.8-0.8 1.9-1.5 3-1.9 2-0.7 4.2-0.6 6.1 0.3s3.4 2.5 4.1 4.6c0.7 2 0.6 4.2-0.3 6.1s-2.5 3.4-4.6 4.1c-4.2 1.5-8.7-0.7-10.2-4.9-0.2-0.5-0.8-0.8-1.3-0.6s-0.8 0.8-0.6 1.3c1.5 4.1 5.4 6.7 9.5 6.7 1.1 0 2.2-0.2 3.3-0.6 5.2-1.8 8-7.5 6.1-12.7z"
                            }),
                        ]),
                    ]),
                ]),

                m(".countdown-buttons", [
                    m("button.big.success", {
                        onclick: function () {
                            App.startTimer(20);
                        }
                    }, "Answer"),
                    m("button.big", {
                        onclick: function () {
                            App.startTimer(20);
                        }
                    }, "Prejump"),
                    m("button", {
                        onclick: function () {
                            App.startTimer(30);
                        }
                    }, "Appeal"),
                    m("button", {
                        onclick: function () {
                            App.startTimer(60);
                        }
                    }, "Time out"),
                ]),
            ]),
        ];
    }
};

var SettingsPage = {
    view: function () {
        return [
            m("header", [
                m("a", {
                    href: "#!/main",
                }, [
                    m("svg.icon", {
                        viewBox: "0 0 24 24"
                    }, [
                        m("path", {
                            d: "M19 11h-11.6l5.3-5.3c0.4-0.4 0.4-1 0-1.4s-1-0.4-1.4 0l-7 7c-0.1 0.1-0.2 0.2-0.2 0.3-0.1 0.2-0.1 0.5 0 0.8 0.1 0.1 0.1 0.2 0.2 0.3l7 7c0.2 0.2 0.5 0.3 0.7 0.3s0.5-0.1 0.7-0.3c0.4-0.4 0.4-1 0-1.4l-5.3-5.3h11.6c0.6 0 1-0.4 1-1s-0.4-1-1-1z"
                        }),
                    ]),
                ]),
            ]),

            m("main#settings", [
                m("div", [
                    m("label", "Theme"),
                    m("select", {
                        value: settings.theme,
                        onchange: function (e) {
                            App.setTheme(e.target.value);
                        },
                    }, [
                        m("option", {
                            value: "light"
                        }, "Light"),
                        m("option", {
                            value: "dark"
                        }, "Dark"),
                        m("option", {
                            value: "joyful"
                        }, "Joyful"),
                    ]),
                ]),

                m("div", [
                    m("label", "Timer sound"),
                    m("select", {
                        value: settings.timerSound,
                        onchange: function (e) {
                            App.setTimerSound(e.target.value);
                        },
                    }, [
                        m("option", {
                            value: "bugaboo"
                        }, "Bugaboo"),
                        m("option", {
                            value: "bumptious"
                        }, "Bumptious"),
                        m("option", {
                            value: "correct"
                        }, "Correct"),
                        m("option", {
                            value: "fanta"
                        }, "Fanta"),
                        m("option", {
                            value: "foofaraw"
                        }, "Foofaraw"),
                        m("option", {
                            value: "ludic"
                        }, "Ludic"),
                        m("option", {
                            value: "tizzy"
                        }, "Tizzy"),
                        m("option", {
                            value: "unctuous"
                        }, "Unctuous"),
                    ]),
                ]),

                m("div", [
                    m("button", {}, "Install offline"),
                    m("br"),
                    m("span.small-text", "Install not available for your device or browser, sorry."),
                    m("span.small-text", "App is installed. Yay!"),
                ]),

                m("div", [
                    m("button", "Self update"),
                ]),

                m("div", [
                    m("span.small-text", "Version 0.1.1"),
                ]),
            ]),
        ];
    }
};

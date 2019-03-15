const settings = {
    timerSound: "correct",
    vibrate: true,

    load() {
        var json = localStorage.getItem("settings");
        if (json) {
            Object.assign(settings, JSON.parse(json));
        }
    },

    save() {
        localStorage.setItem("settings", JSON.stringify(settings));
    },

    reset() {
        localStorage.removeItem("settings");
        location.reload();
    },
};

settings.load();

export default settings;

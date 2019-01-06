var state = {
    active: false,
    startTime: 0,
    audio: null,
}

var sounds = [
    {
        name: "Sweet Alert 1",
        path: "sounds/72125__kizilsungur__sweetalertsound1.wav",
    },
    {
        name: "Sweet Alert 3",
        path: "sounds/72127__kizilsungur__sweetalertsound3.wav",
    },
    {
        name: "Sweet Alert 4",
        path: "sounds/72128__kizilsungur__sweetalertsound4.wav",
    },
    {
        name: "Sweet Alert 5",
        path: "sounds/72129__kizilsungur__sweetalertsound5.wav",
    },
    {
        name: "Sweet Alert",
        path: "sounds/204369__philitup321__alert-sound.ogg",
    },
    {
        name: "Foofaraw",
        path: "sounds/276607__mickleness__ringtone-foofaraw.wav",
    },
    {
        name: "Fanta",
        path: "sounds/276608__mickleness__ringtone-fanta.wav",
    },
    {
        name: "Bumptious",
        path: "sounds/276609__mickleness__notification-bumptious.wav",
    },
    {
        name: "Bugaboo",
        path: "sounds/276610__mickleness__notification-bugaboo.wav",
    },
    {
        name: "Unctuous",
        path: "sounds/276611__mickleness__notification-unctuous.wav",
    },
    {
        name: "Tizzy",
        path: "sounds/276612__mickleness__ringtone-tizzy.wav",
    },
    {
        name: "Ludic",
        path: "sounds/276614__mickleness__ludic.wav",
    },
    {
        name: "Correct",
        path: "sounds/415762__thebuilder15__notification-correct.wav",
    },
];

var timerValueLabel = document.querySelector(".countdown-display .countdown-text");
var countdownCircle = document.querySelector(".countdown-display .countdown-circle");
var soundsSelectBox = document.querySelector("#sounds-select-box");


var ui = {
    showPage: function (name) {
        document.querySelector("main > section#" + name).classList.add("is-active");
        document.querySelector("main > section:not(#" + name + ")").classList.remove("is-active");
    },

    /**
     * This function gets invoked frequently to update the current state of the UI.
     */
    update: function () {
        if (state.active) {
            var total = state.endTime - state.startTime;
            var remaining = Math.max(0, state.endTime - performance.now());

            ui.setCountdownValue(remaining);
            ui.setCircleFraction(remaining / total);

            if (remaining <= 0) {
                state.active = false;
                app.playSound();
            }
        } else {
            ui.setCountdownValue(0);
            ui.setCircleFraction(1);
        }
    },

    setCountdownValue: function (value) {
        var text = (value / 1000).toFixed(1);

        if (timerValueLabel.textContent != text) {
            timerValueLabel.textContent = text;
        }
    },

    setCircleFraction: function (fraction) {
        countdownCircle.setAttribute("stroke-dasharray", (90 * Math.PI * fraction) + " 1000");
    },
}

window.app = {
    showTimer: function () {
        ui.showPage("timer");
    },

    showSettings: function () {
        ui.showPage("settings");
    },

    start: function (seconds) {
        state.startTime = performance.now();
        state.endTime = state.startTime + (seconds * 1000);
        state.active = true;
    },

    cancel: function () {
        state.active = false;
    },

    playSound: function () {
        state.audio.play();
    },

    init: function () {
        for (var i = 0; i < sounds.length; ++i) {
            var option = document.createElement("option");
            option.value = i;
            option.innerText = sounds[i].name;
            soundsSelectBox.appendChild(option);
        }

        soundsSelectBox.addEventListener("change", function () {
            state.audio = new Audio(sounds[parseInt(this.value, 10)].path);
        });

        state.audio = new Audio(sounds[0].path);
        window.setInterval(ui.update, 50);
    },
}

app.init();


document.addEventListener('DOMContentLoaded', () => {

    // Get all "navbar-burger" elements
    const $navbarBurgers = Array.prototype.slice.call(document.querySelectorAll('.navbar-burger'), 0);

    // Check if there are any navbar burgers
    if ($navbarBurgers.length > 0) {

      // Add a click event on each of them
      $navbarBurgers.forEach( el => {
        el.addEventListener('click', () => {

          // Get the target from the "data-target" attribute
          const target = el.dataset.target;
          const $target = document.getElementById(target);

          // Toggle the "is-active" class on both the "navbar-burger" and the "navbar-menu"
          el.classList.toggle('is-active');
          $target.classList.toggle('is-active');

        });
      });
    }

  });

import {MainPage, SettingsPage} from "./components";
import * as audio from "./audio";
import settings from "./settings";
import state from "./state";


declare var m: any;
declare var Promise: any;


export function update() {
    if ("caches" in window) {
        caches.delete("static").then(() => {
            location.reload();
        });
    } else {
        location.reload();
    }
}

export function startTimer(seconds: number) {
    state.timer.startTime = performance.now();
    state.timer.endTime = state.timer.startTime + (seconds * 1000);
    state.timer.active = true;
}

export function cancelTimer() {
    state.timer.active = false;
    state.timer.remaining = 0;
}

function tick() {
    if (state.timer.active) {
        let remaining = Math.max(0, state.timer.endTime - performance.now());
        state.timer.remaining = remaining;

        if (remaining <= 0) {
            state.timer.active = false;
            audio.play(settings.timerSound);
        }

        m.redraw();
    }
}

function init() {
    // Set up offline cache handling.
    if ("serviceWorker" in navigator) {
        navigator.serviceWorker.register("service-worker.js");
    }

    setInterval(tick, 50);

    function page(component: object) {
        return {
            onmatch: () => {
                let previousPage = document.querySelector("body > main") as HTMLElement;
                if (previousPage) {
                    previousPage.style.opacity = "0";
                }

                return new Promise((resolve: (v: any) => void) => {
                    setTimeout(() => {
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
}

init();

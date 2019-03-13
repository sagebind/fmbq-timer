import * as audio from "./audio";
import settings from "./settings";
import timer from "./timer";
import MainPage from "./components/main-page";
import SettingsPage from "./components/settings-page";

declare var m: any;

export function update() {
    if ("caches" in window) {
        caches.delete("static").then(() => {
            location.reload();
        });
    } else {
        location.reload();
    }
}

function tick() {
    if (timer.active) {
        if (timer.remaining <= 0) {
            audio.play(settings.timerSound);
            timer.reset();
        }

        m.redraw();
    }
}

function pageDecorator<T>(component: T) {
    return {
        onmatch(): Promise<T> {
            let previousPage = document.querySelector("body > main") as HTMLElement;
            if (previousPage) {
                previousPage.style.opacity = "0";
            }

            return new Promise(resolve => {
                setTimeout(() => resolve(component), 200);
            });
        }
    };
}

// Set up offline cache handling.
if ("serviceWorker" in navigator) {
    navigator.serviceWorker.register("service-worker.js");
}

m.route(document.body, "/", {
    "/": pageDecorator(MainPage),
    "/settings": pageDecorator(SettingsPage),
});

setInterval(tick, 50);

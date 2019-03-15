import * as app from "../app";
import * as audio from "../audio";
import * as install from "../install";
import settings from "../settings";
import {BackIcon} from "./icons";

declare const m: any;

export default {
    view() {
        let installMessage = "";

        if (install.isInstalled()) {
            installMessage = "App is installed. Yay!";
        } else if (!install.supported()) {
            installMessage = "Install not available for your device or browser, sorry.";
        }

        return [
            m("header", [
                m("a", {
                    href: "#!/",
                }, m(BackIcon)),
            ]),

            m("main#settings", [
                m("div", [
                    m("label", "Timer sound"),
                    m("select", {
                        value: settings.timerSound,
                        onchange: function (e: Event) {
                            settings.timerSound = (e.target as HTMLInputElement).value;
                            audio.play(settings.timerSound);
                            settings.save();
                        },
                    }, audio.sounds.map(sound => {
                        return m("option", {
                            value: sound.id
                        }, sound.name);
                    })),
                    m("a.button", {href: "#!/settings/sounds"}, "Sounds"),
                ]),

                m("div", [
                    m("button", {
                        onclick: () => settings.reset()
                    }, "Reset settings"),
                ]),

                m("div", [
                    m("button", {
                        disabled: install.isInstalled() || !install.supported(),
                        onclick: () => install.install(),
                    }, "Install offline"),
                    m("br"),
                    m("span.small-text", installMessage),
                ]),

                m("div", [
                    m("button", {
                        onclick: () => app.update()
                    }, "Self update"),
                ]),

                m("div", [
                    m("span.small-text", "Version 0.3.0"),
                ]),
            ]),
        ];
    }
};

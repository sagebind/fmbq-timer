import * as app from "../app";
import * as alarm from "../alarm";
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
                m("h1", "Settings")
            ]),

            m("main#settings", m("fieldset", [
                m("label[for='setting-sounds']", [
                    m("input#setting-sounds[type='checkbox']", {
                        checked: settings.soundEnabled,
                        onchange(e: Event) {
                            settings.soundEnabled = (e.target as HTMLInputElement).checked;
                            settings.save();
                        }
                    }),
                    "Enable sounds",
                ]),

                m("button", {
                    disabled: !settings.soundEnabled,
                    onclick() {
                        location.href = "#!/settings/sounds";
                    }
                }, [
                    m("div", "Timer sound"),
                    m("small", audio.sounds.find(s => s.id == settings.timerSound).name),
                ]),

                m("label[for='setting-vibrate']", [
                    m("input#setting-vibrate[type='checkbox']", {
                        checked: settings.vibrateEnabled,
                        onchange(e: Event) {
                            settings.vibrateEnabled = (e.target as HTMLInputElement).checked;
                            alarm.triggerVibrate();
                            settings.save();
                        }
                    }),
                    "Enable vibration",
                ]),

                m("button", {
                    onclick: () => settings.reset()
                }, "Reset settings"),

                m("button", {
                    disabled: install.isInstalled() || !install.supported(),
                    onclick: () => install.install(),
                }, [
                    m("div", "Install offline"),
                    m("small", installMessage),
                ]),

                m("button", {
                    onclick: () => app.update()
                }, "Check for updates"),

                m("div", m("small", "Version 0.4.0")),
            ])),
        ];
    }
};

import * as app from "../app";
import * as audio from "../audio";
import * as install from "../install";
import settings from "../settings";

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

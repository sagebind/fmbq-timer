import timer from "../timer";
import CountdownButtons from "./countdown-buttons";
import CountdownCircle from "./countdown-circle";
import ResetButton from "./reset-button";
import {SettingsIcon} from "./icons";

declare const m: any;

export default {
    view() {
        return [
            m("header", [
                m("h1", "FMBQ Timer"),
                m("a.right[title=Settings]", {
                    href: "#!/settings",
                }, m(SettingsIcon)),
            ]),
            m("main#timer", {
                style: {
                    display: "flex",
                    "flex-direction": "column",
                    "align-items": "center",
                    "justify-content": "space-between",
                    padding: "1rem",
                }
            }, [
                m(".countdown-display", {
                    style: {
                        position: "relative",
                        "align-self": "center",
                        "flex-grow": 1
                    }
                }, [
                    m(CountdownCircle),
                    m(ResetButton, {
                        disabled: !timer.active
                    }),
                ]),
                m(CountdownButtons),
            ]),
        ];
    }
};

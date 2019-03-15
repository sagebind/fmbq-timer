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
                m(".brand", "FMBQ Timer"),
                m("a.right[title=Settings]", {
                    href: "#!/settings",
                }, m(SettingsIcon)),
            ]),
            m("main#timer", [
                m(".countdown-display", [
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

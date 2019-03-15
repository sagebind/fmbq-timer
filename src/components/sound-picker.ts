import * as audio from "../audio";
import settings from "../settings";
import {BackIcon} from "./icons";

declare const m: any;

const Item = {
    view(vnode) {
        return m(
            "button",
            {
                className: settings.timerSound === vnode.attrs.sound.id ? "selected" : "",

                onclick() {
                    audio.play(vnode.attrs.sound.id);
                    settings.timerSound = vnode.attrs.sound.id;
                    settings.save();
                }
            },
            vnode.attrs.sound.name
        );
    }
}

export default {
    view() {
        return [
            m("header", [
                m("a", {
                    href: "#!/settings",
                }, m(BackIcon)),
            ]),

            m("main", [
                m("fieldset", audio.sounds.map(sound => m(Item, {
                    sound: sound
                })))
            ]),
        ];
    }
}

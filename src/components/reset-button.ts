import timer from "../timer";
import {ResetIcon} from "./icons";

declare const m: any;

export default {
    view(vnode) {
        return m("button.round.reset-button.warning[title=Cancel]", {
            disabled: vnode.attrs.disabled,
            onclick: () => timer.reset(),
        }, m(ResetIcon));
    }
};


import timer from "../timer";

declare const m: any;

export default {
    view() {
        return m(".countdown-buttons", [
            m("button.big.success", {
                onclick: () => timer.start(20)
            }, "Answer"),
            m("button.big", {
                onclick: () => timer.start(20)
            }, "Prejump"),
            m("button", {
                onclick: () => timer.start(30)
            }, "Appeal"),
            m("button", {
                onclick: () => timer.start(60)
            }, "Time out"),
        ]);
    }
};

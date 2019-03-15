import timer from "../timer";

declare const m: any;

export default {
    view() {
        return m(".countdown-buttons", {
            style: {
                display: "flex",
                "flex-wrap": "wrap",
            }
        }, [
            m("button.button.big.success", {
                onclick: () => timer.start(20)
            }, "Answer"),
            m("button.button.big", {
                onclick: () => timer.start(20)
            }, "Prejump"),
            m("button.button", {
                onclick: () => timer.start(30)
            }, "Appeal"),
            m("button.button", {
                onclick: () => timer.start(60)
            }, "Time out"),
        ]);
    }
};

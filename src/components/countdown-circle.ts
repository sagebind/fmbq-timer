import timer from "../timer";

declare const m: any;

function countdownCircleStyle() {
    if (timer.active) {
        var fraction = timer.remaining / timer.duration;
        return (90 * Math.PI * fraction) + " 300";
    }

    return "300 300";
}

export default {
    view() {
        var timerText = (timer.remaining / 1000).toFixed(1);

        return m("svg", {
            viewBox: "0 0 100 100",
            style: {
                display: "block",
                width: "100%",
                "max-height": "65vh",
            },
            onclick: () => timer.start(20)
        }, [
            m("circle.countdown-circle", {
                cx: 50,
                cy: 50,
                r: 45,
                fill: "none",
                stroke: "currentColor",
                transform: "rotate(-90, 50, 50)",
                "stroke-dasharray": countdownCircleStyle(),
                style: {
                    "transition-property": "stroke-dasharray"
                }
            }),
            m("text.countdown-text", {
                x: 50,
                y: 50,
                fill: "currentColor",
                "dominant-baseline": "central",
                style: {
                    "font-size": "32px",
                    "text-anchor": "middle"
                }
            }, timerText),
        ]);
    }
};

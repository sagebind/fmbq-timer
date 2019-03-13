export class Timer {
    active = false;
    private startTime = 0;
    private endTime = 0;

    get duration() {
        return this.endTime - this.startTime;
    }

    get remaining() {
        if (this.active) {
            return Math.max(0, this.endTime - performance.now());
        }
        return 0;
    }

    start(seconds: number) {
        this.startTime = performance.now();
        this.endTime = this.startTime + (seconds * 1000);
        this.active = true;
    }

    reset() {
        this.active = false;
    }
}

export default new Timer();

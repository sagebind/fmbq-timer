import * as audio from './audio';
import settings from './settings';

export function trigger() {
    triggerVibrate();
    triggerSound();
}

export function triggerSound() {
    if (settings.soundEnabled) {
        audio.play(settings.timerSound);
    }
}

export function triggerVibrate() {
    if (settings.vibrateEnabled && "vibrate" in navigator) {
        if (navigator.vibrate(0)) {
            navigator.vibrate(100);
        }
    }
}

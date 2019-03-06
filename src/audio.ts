let sounds = new Map<string, HTMLAudioElement>();

export function play(sound: string) {
    let audio: HTMLAudioElement;

    if (sounds.has(sound)) {
        audio = sounds.get(sound);
    } else {
        audio = new Audio("sounds/" + sound + ".wav");
        sounds.set(sound, audio);
    }

    sounds.forEach(value => {
        value.pause();
    });

    audio.currentTime = 0;
    audio.play();
}

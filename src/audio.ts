export class Sound {
    name: string;
    audio: HTMLAudioElement;

    constructor(name: string) {
        this.name = name;
    }

    get id() {
        return this.name.toLowerCase().replace(/\s/g, "-");
    }

    get uri() {
        return `sounds/${this.id}.wav`;
    }

    play() {
        if (!this.audio) {
            this.audio = new Audio(this.uri);
        }

        this.audio.currentTime = 0;
        this.audio.play();
    }

    pause() {
        if (this.audio) {
            this.audio.pause();
        }
    }
}

export const sounds = [
    new Sound("Bleep"),
    new Sound("Bumptious"),
    new Sound("Chime"),
    new Sound("Correct"),
    new Sound("Energy"),
    new Sound("Foofaraw"),
    new Sound("MSN"),
    new Sound("Sleigh Bells"),
    new Sound("Tizzy"),
];

export function play(id: string) {
    sounds.forEach(sound => sound.pause());

    let sound = sounds.find(sound => sound.id == id);

    if (sound) {
        sound.play();
    } else {
        console.log("Unknown sound: " + id);
    }
}

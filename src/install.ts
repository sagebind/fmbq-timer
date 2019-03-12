declare var m: any;

var installed = !!navigator["standalone"] ||
    window.matchMedia("(display-mode: standalone)").matches;
var installPrompt: any;

export function supported(): boolean {
    return 'BeforeInstallPromptEvent' in window || isIOS();
}

export function isInstalled(): boolean {
    return installed;
}

export function install() {
    if (installPrompt) {
        installPrompt.prompt();
        installPrompt.userChoice.then(result => {
            if (result.outcome === "accepted") {
                installed = true;
            }
            installPrompt = null;
        });
    } else if (isIOS()) {
        alert("To install, click the Share icon and select \"Add to Home Screen\".");
    } else {
        alert("There was a problem when trying to install.");
    }
}

function isIOS(): boolean {
    return /iPad|iPhone|iPod/.test(navigator.userAgent);
}

window.addEventListener("beforeinstallprompt", (e) => {
    e.preventDefault();
    installPrompt = e;
});

window.addEventListener("appinstalled", () => {
    installed = true;
    m.redraw();
});

export default function initializer() {
    return {
        onStart: () => {
            document.querySelector("#loading").style.display = "block";
        },
        onProgress: ({ current, total }) => {
            if (total > 0) {
                const percent = (current / total) * 100;
                document.querySelector("#loading-text").innerText = `Loading... ${Math.round(percent)}%`;
            }
        },
        onComplete: () => {
            document.querySelector("#loading").style.display = "none";
        },
        onFailure: (error) => {
            console.error("Failed to load WASM:", error);
            document.querySelector("#loading-text").innerText = "Failed to load game.";
        }
    };
}

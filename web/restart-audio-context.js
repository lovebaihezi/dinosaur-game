// Audio context resumption for browser autoplay policy compliance
// Based on https://developer.chrome.com/blog/web-audio-autoplay/#moving-forward
(function () {
  // An array of all contexts to resume on the page
  const audioContextList = [];

  // An array of various user interaction events we should listen for
  const userInputEventNames = [
    "click",
    "contextmenu",
    "auxclick",
    "dblclick",
    "mousedown",
    "mouseup",
    "pointerup",
    "touchend",
    "touchstart",
    "keydown",
    "keyup",
  ];

  // Helper function to create a proxy for AudioContext constructors
  function createAudioContextProxy(OriginalAudioContext) {
    if (!OriginalAudioContext) return null;
    return new Proxy(OriginalAudioContext, {
      construct(target, args) {
        const result = new target(...args);
        audioContextList.push(result);
        return result;
      },
    });
  }

  // Proxy AudioContext (standard)
  if (self.AudioContext) {
    self.AudioContext = createAudioContextProxy(self.AudioContext);
  }

  // Proxy webkitAudioContext (Safari compatibility)
  if (self.webkitAudioContext) {
    self.webkitAudioContext = createAudioContextProxy(self.webkitAudioContext);
  }

  // To resume all AudioContexts being tracked
  function resumeAllContexts(_event) {
    let allRunning = true;

    audioContextList.forEach((context) => {
      if (context.state !== "running") {
        allRunning = false;
        context.resume().catch((err) => {
          // Log non-critical errors - context might already be closed or not allowed
          console.debug("AudioContext resume skipped:", err.message || err);
        });
      }
    });

    // Only remove listeners if we have contexts and all are running
    // Keep listeners active if no contexts exist yet (WASM might not have loaded)
    if (audioContextList.length > 0 && allRunning) {
      userInputEventNames.forEach((eventName) => {
        document.removeEventListener(eventName, resumeAllContexts);
      });
    }
  }

  // We bind the resume function for each user interaction
  // event on the page
  userInputEventNames.forEach((eventName) => {
    document.addEventListener(eventName, resumeAllContexts);
  });
})();

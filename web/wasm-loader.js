/**
 * WASM Loader Initializer for Trunk
 * Handles loading screen display and error handling during WASM initialization.
 * Uses Trunk's initializer framework with modern ESM syntax.
 */

const LOADING_TIMEOUT_MS = 60000; // 60 seconds

/**
 * Hides the loading screen element.
 */
const hideLoadingScreen = () => {
  const loadingScreen = document.getElementById('loading-screen');
  if (loadingScreen) {
    loadingScreen.style.display = 'none';
  }
};

/**
 * Shows the error popup with the given error message.
 * @param {string|Error} error - The error to display
 */
const showErrorPopup = (error) => {
  const loadingScreen = document.getElementById('loading-screen');
  const errorOverlay = document.getElementById('error-overlay');
  const errorPopup = document.getElementById('error-popup');
  const errorDetails = document.getElementById('error-details');

  if (loadingScreen) {
    loadingScreen.style.display = 'none';
  }
  if (errorOverlay) {
    errorOverlay.style.display = 'block';
  }
  if (errorPopup) {
    errorPopup.style.display = 'block';
  }
  if (errorDetails && error) {
    errorDetails.textContent = error.toString();
  }
};

/**
 * Trunk initializer function.
 * Returns an object with lifecycle hooks for WASM loading.
 * @returns {Object} Initializer hooks object
 */
export default function wasmLoaderInitializer() {
  let loadingTimeout = null;

  return {
    /**
     * Called when WASM loading starts.
     */
    onStart: () => {
      // Set up timeout to show error if loading takes too long
      loadingTimeout = setTimeout(() => {
        const loadingScreen = document.getElementById('loading-screen');
        if (loadingScreen && loadingScreen.style.display !== 'none') {
          showErrorPopup('Loading timed out. Your browser may not support WebAssembly or WebGPU.');
        }
      }, LOADING_TIMEOUT_MS);
    },

    /**
     * Called with loading progress updates.
     * @param {Object} progress - Progress object with current and total
     */
    onProgress: ({ current, total }) => {
      // Optional: Update progress indicator if needed
      // Could add a progress bar in the future
      if (total > 0) {
        const percent = Math.round((current / total) * 100);
        console.debug(`WASM loading: ${percent}%`);
      }
    },

    /**
     * Called when WASM loading completes (success or failure).
     */
    onComplete: () => {
      if (loadingTimeout) {
        clearTimeout(loadingTimeout);
        loadingTimeout = null;
      }
    },

    /**
     * Called when WASM loads successfully.
     * @param {WebAssembly.Module} _wasm - The loaded WASM module
     */
    onSuccess: (_wasm) => {
      hideLoadingScreen();
    },

    /**
     * Called when WASM loading fails.
     * @param {Error} error - The error that occurred
     */
    onFailure: (error) => {
      showErrorPopup(error);
    }
  };
}

(function () {
    // Unique ID to prevent creating multiple bars on reloads/events
    const BAR_ID = 'tauri-custom-drag-bar';

    function injectBar() {
        if (document.getElementById(BAR_ID)) return; // Already exists

        const dragBar = document.createElement('div');
        dragBar.id = BAR_ID;
        dragBar.style.cssText = `
            position: fixed;
            top: 0;
            left: 0;
            right: 0;
            height: 30px;
            z-index: 999999;
            -webkit-app-region: drag;
            pointer-events: auto;
            cursor: grab;
        `;
        document.body.appendChild(dragBar);
    }

    // If DOM is ready, run now. Otherwise wait.
    if (document.readyState === 'loading') {
        window.addEventListener('DOMContentLoaded', injectBar);
    } else {
        injectBar();
    }
})();
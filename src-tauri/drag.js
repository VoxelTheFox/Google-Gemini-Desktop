(function () {
    const BAR_ID = 'gemini-drag-signal';

    function injectDragBar() {
        if (document.getElementById(BAR_ID)) return;

        // 1. Create the invisible drag area
        const dragBar = document.createElement('div');
        dragBar.id = BAR_ID;
        dragBar.style.cssText = `
            position: fixed;
            top: 0;
            left: 0;
            right: 0;
            height: 32px;
            z-index: 2147483647; /* Max Z-Index */
            cursor: grab;
            background-color: transparent; /* Invisible */
        `;

        // 2. The Logic: On Click, tell Rust to "Start Dragging"
        dragBar.addEventListener('mousedown', (e) => {
            // Only Left Click
            if (e.button === 0) {
                if (window.__TAURI__ && window.__TAURI__.core) {
                    // Send a signal to the Rust backend
                    window.__TAURI__.core.invoke('start_drag');
                } else {
                    console.error("Tauri API not ready");
                }
            }
        });

        document.documentElement.appendChild(dragBar);

        // 3. Ensure Rounded Corners (Since we removed decorations)
        const radius = "12px"; 
        document.documentElement.style.borderRadius = radius;
        document.documentElement.style.overflow = "hidden";
        document.body.style.borderRadius = radius;
        document.body.style.overflow = "hidden";
    }

    if (document.readyState === 'loading') {
        window.addEventListener('DOMContentLoaded', injectDragBar);
    } else {
        injectDragBar();
    }
})();
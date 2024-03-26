import { h, render } from 'https://esm.sh/preact';
import { useState, useRef, useEffect } from 'https://esm.sh/preact/hooks';
import htm from 'https://esm.sh/htm';

// Initialize htm with Preact
const html = htm.bind(h);

function App() {
    useEffect(() => {
    }, []);

    return html`<div>BLE Amp</div>`;
}
render(html`<${App} />`, document.body);

/*
Components
*/

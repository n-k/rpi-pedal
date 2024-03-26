import { h, render } from 'https://esm.sh/preact';
import { useState, useRef, useEffect } from 'https://esm.sh/preact/hooks';
import htm from 'https://esm.sh/htm';

// Initialize htm with Preact
const html = htm.bind(h);

function App() {
    useEffect(() => {
    }, []);

    return html`<${Device} />`;
}
render(html`<${App} />`, document.body);

/*
Components
*/
function Device() {
    const [ch, setCh] = useState();
    const [config, setConfig] = useState();

    useEffect(() => {
        log();
    }, [ch]);

    const connect = () => {
        navigator.bluetooth.requestDevice({
            filters: [
                { services: ['00000000-0000-0000-0000-0000feedc0de'] }
            ]
        })
            .then(device => {
                console.log(device);
                return device.gatt.connect();
            })
            .then(server => {
                console.log('server', server);
                return server.getPrimaryServices();
            })
            .then(services => {
                console.log('services', services);
                const service = services[0];
                return service.getCharacteristics();
            })
            .then(chars => {
                console.log('chars', chars);
                const characteristic = chars[0];
                console.log(characteristic);
                setCh(characteristic);
            })
            .catch(error => { console.error(error); });
    };

    const log = () => {
        if (!ch) return;

        ch.readValue()
            .then(v => {
                const str = new TextDecoder().decode(v.buffer);
                console.log('value: ', v, `as string: ${str}`);
                setConfig(JSON.parse(str));
            })
            .catch(error => {
                console.error(error);
            })
    };

    const write = (config) => {
        const enc = new TextEncoder();
        ch.writeValue(enc.encode(JSON.stringify(config)))
            .then(() => {
                log();
            })
            .catch(error => {
                console.error(error);
            });
    };

    if (!ch) {
        return html`<div>
            <button onClick=${() => connect()}>Connect</button>
        </div>`
    }

    if (!config) {
        return html`<div>
            Loading...
        </div>`
    }
    // we have a config

    return html`<div>
        <div>
            Gain
        <div>
        <div>
            <input 
                type="range" min="0" max="255" value=${config.gain}
                onChange=${(e) => {
            console.log(e.target.value);
            setConfig({ gain: parseInt(e.target.value) });
            write({ gain: parseInt(e.target.value) });
        }}
            />
        </div>
    </div>`
}

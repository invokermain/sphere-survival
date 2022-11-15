import init, { main } from './pkg/wasm.js';

const elementTargetButton = document.querySelector('#button-start')
const elementMain = document.querySelector('#main')

async function run() {
    elementTargetButton.removeEventListener('click', run)
    elementMain.remove()

    const context = new AudioContext()

    if (context.state !== 'running') {
        await context.resume()
    }

    await init()

    main()
}

elementTargetButton.addEventListener('click', run, {
    once: true,
    passive: true,
})
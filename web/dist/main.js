import init, { run_game } from './pkg/wasm.js';



async function run() {
    elementTargetButton.removeEventListener('click', run)
    elementmain.remove()

    const context = new AudioContext()

    if (context.state !== 'running') {
        await context.resume()
    }

    await init()

    run_game()
}

elementTargetButton.addEventListener('click', run, {
    once: true,
    passive: true,
})
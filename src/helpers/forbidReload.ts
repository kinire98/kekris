export default function forbidReload() {
    window.addEventListener('keydown', (e: KeyboardEvent) => {
        if (
            e.key === 'F5' ||
            (e.ctrlKey && e.key.toLowerCase() === 'r')
        ) {
            e.preventDefault();
        }
    });

}
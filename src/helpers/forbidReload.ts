/**
 * Prevents the user from reloading the page using the keyboard.
 *
 * This function listens for the F5 key or Ctrl+R key combination and prevents the default action,
 * effectively disabling the reload functionality.
 */
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
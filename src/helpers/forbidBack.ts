/**
 * Prevents the user from navigating back to the previous page using the browser's back button.
 *
 * This function pushes a new state onto the history stack and listens for the popstate event,
 * pushing another state whenever the user tries to navigate back.
 */
export default function forbidBack() {
    history.pushState(null, document.title, location.href);
    window.addEventListener('popstate', function () {
        history.pushState(null, document.title, location.href);
    });
}
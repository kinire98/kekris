const usernameKey = "username";
/**
 * Sets the username in local storage.
 * @param username The username to set.
 */
export function setUsername(username: string) {
    localStorage.setItem(usernameKey, username);
}

/**
 * Gets the username from local storage.
 * @returns The username, or null if no username is set.
 */
export function getUsername(): string | null {
    return localStorage.getItem(usernameKey);
}
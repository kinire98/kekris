const usernameKey = "username";
export function setUsername(username: string) {
    localStorage.setItem(usernameKey, username);
}

export function getUsername(): string | null {
    return localStorage.getItem(usernameKey);
}
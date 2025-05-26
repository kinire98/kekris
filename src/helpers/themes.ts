const themeKey = "theme-key-ls";

/**
 * Gets the current theme from local storage.
 * @returns The theme name, or null if no theme is set.
 */
export function getTheme(): string | null {
    return localStorage.getItem(themeKey);
}

/**
 * Sets the current theme in local storage.
 * @param value The theme name to set.
 */
export function setTheme(value: string) {
    localStorage.setItem(themeKey, value);
}
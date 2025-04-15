const themeKey = "theme-key-ls";


export function getTheme(): string | null {
    return localStorage.getItem(themeKey);
}

export function setTheme(value: string) {
    localStorage.setItem(themeKey, value);
}
const defaultLocaleKey = "defaultLocaleKey";

/**
 * Gets the default locale from local storage or the navigator.
 * If no locale is found, it defaults to 'es' if the navigator language is Spanish, otherwise it defaults to 'en'.
 * @returns The default locale.
 */
export function getDefaultLocale(): string {
    let lsItem = localStorage.getItem(defaultLocaleKey);
    if (lsItem != null) {
        return lsItem;
    }
    const language = navigator.language.split("-")[0];
    if (language == 'es') {
        setDefaultLocale('es');
        return 'es';
    }
    setDefaultLocale('en');
    return 'en';
}

/**
 * Sets the default locale in local storage.
 * @param defaultLocale The default locale to set.
 */
export function setDefaultLocale(defaultLocale: string) {
    localStorage.setItem(defaultLocaleKey, defaultLocale);
}
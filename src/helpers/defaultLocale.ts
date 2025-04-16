const defaultLocaleKey = "defaultLocaleKey";


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
export function setDefaultLocale(defaultLocale: string) {
    localStorage.setItem(defaultLocaleKey, defaultLocale);
}
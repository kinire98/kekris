import { ui, defaultLang } from './ui';

export function getLangFromUrl(url: URL) {
    const lang = url.pathname.split('/');
    for (const key in ui) {
        if (key == lang[2]) {
            return lang[2] as keyof typeof ui
        }
    }
    return defaultLang;
}

export function useTranslations(lang: keyof typeof ui) {
    return function t(key: keyof typeof ui[typeof defaultLang]) {
        return ui[lang][key] || ui[defaultLang][key];
    }
}

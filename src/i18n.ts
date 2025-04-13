import { createI18n } from "vue-i18n";
function loadLocaleMessages() {
    const messages: Record<string, any> = {}

    const files = import.meta.glob('./locales/**/**/*.json', { eager: true })

    for (const path in files) {
        const matched = path.match(/\.\/locales\/([a-z0-9-_]+)\/.+\.json$/i)
        if (matched) {
            const locale = matched[1]
            messages[locale] ??= {}
            Object.assign(messages[locale], (files[path] as any).default)
        }
    }

    return messages
}
export default createI18n({
    locale: "es",
    fallbackLocale: "en",
    messages: loadLocaleMessages(),
    legacy: false
})
// @ts-check
import { defineConfig } from 'astro/config';
import icon from "astro-icon";

// https://astro.build/config
export default defineConfig({
  site: 'https://kinire98.github.io',
  base: 'kekris',
  integrations: [icon()],
  i18n: {
    defaultLocale: 'en',
    locales: ['en', 'es']
  }
});

import i18n from "i18next";
import { initReactI18next } from "react-i18next";

import type { Language } from "@/domain/entities/config";

import { en } from "./locales/en";
import { es } from "./locales/es";

void i18n.use(initReactI18next).init({
  resources: { es, en },
  lng: "es",
  fallbackLng: "en",
  interpolation: { escapeValue: false },
});

export function setLanguage(lang: Language): void {
  void i18n.changeLanguage(lang);
}

export default i18n;

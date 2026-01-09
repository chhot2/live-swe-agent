/**
 * Internationalization (i18n) Module
 * 
 * Provides multi-language support including:
 * - Language detection
 * - Translation management
 * - RTL support
 * - Locale-specific formatting
 */

import { writable, derived, get } from 'svelte/store';
import { persist } from './persist';

/**
 * Supported languages
 */
export type SupportedLanguage = 'en' | 'es' | 'fr' | 'de' | 'zh' | 'ja' | 'ko' | 'pt' | 'ru' | 'ar';

/**
 * Language metadata
 */
export interface LanguageInfo {
  code: SupportedLanguage;
  name: string;
  nativeName: string;
  rtl: boolean;
  dateFormat: string;
  numberFormat: {
    decimal: string;
    thousand: string;
  };
}

/**
 * Available languages with metadata
 */
export const languages: Record<SupportedLanguage, LanguageInfo> = {
  en: {
    code: 'en',
    name: 'English',
    nativeName: 'English',
    rtl: false,
    dateFormat: 'MM/DD/YYYY',
    numberFormat: { decimal: '.', thousand: ',' },
  },
  es: {
    code: 'es',
    name: 'Spanish',
    nativeName: 'Español',
    rtl: false,
    dateFormat: 'DD/MM/YYYY',
    numberFormat: { decimal: ',', thousand: '.' },
  },
  fr: {
    code: 'fr',
    name: 'French',
    nativeName: 'Français',
    rtl: false,
    dateFormat: 'DD/MM/YYYY',
    numberFormat: { decimal: ',', thousand: ' ' },
  },
  de: {
    code: 'de',
    name: 'German',
    nativeName: 'Deutsch',
    rtl: false,
    dateFormat: 'DD.MM.YYYY',
    numberFormat: { decimal: ',', thousand: '.' },
  },
  zh: {
    code: 'zh',
    name: 'Chinese',
    nativeName: '中文',
    rtl: false,
    dateFormat: 'YYYY-MM-DD',
    numberFormat: { decimal: '.', thousand: ',' },
  },
  ja: {
    code: 'ja',
    name: 'Japanese',
    nativeName: '日本語',
    rtl: false,
    dateFormat: 'YYYY/MM/DD',
    numberFormat: { decimal: '.', thousand: ',' },
  },
  ko: {
    code: 'ko',
    name: 'Korean',
    nativeName: '한국어',
    rtl: false,
    dateFormat: 'YYYY-MM-DD',
    numberFormat: { decimal: '.', thousand: ',' },
  },
  pt: {
    code: 'pt',
    name: 'Portuguese',
    nativeName: 'Português',
    rtl: false,
    dateFormat: 'DD/MM/YYYY',
    numberFormat: { decimal: ',', thousand: '.' },
  },
  ru: {
    code: 'ru',
    name: 'Russian',
    nativeName: 'Русский',
    rtl: false,
    dateFormat: 'DD.MM.YYYY',
    numberFormat: { decimal: ',', thousand: ' ' },
  },
  ar: {
    code: 'ar',
    name: 'Arabic',
    nativeName: 'العربية',
    rtl: true,
    dateFormat: 'DD/MM/YYYY',
    numberFormat: { decimal: '٫', thousand: '٬' },
  },
};

/**
 * Translation dictionary type
 */
type TranslationDict = Record<string, string | Record<string, string>>;

/**
 * English translations (default)
 */
const en: TranslationDict = {
  // Common
  'common.ok': 'OK',
  'common.cancel': 'Cancel',
  'common.save': 'Save',
  'common.delete': 'Delete',
  'common.edit': 'Edit',
  'common.close': 'Close',
  'common.search': 'Search',
  'common.loading': 'Loading...',
  'common.error': 'Error',
  'common.success': 'Success',
  'common.warning': 'Warning',
  'common.confirm': 'Confirm',
  'common.yes': 'Yes',
  'common.no': 'No',
  
  // Navigation
  'nav.back': 'Back',
  'nav.forward': 'Forward',
  'nav.reload': 'Reload',
  'nav.stop': 'Stop',
  'nav.home': 'Home',
  
  // Tabs
  'tabs.newTab': 'New Tab',
  'tabs.closeTab': 'Close Tab',
  'tabs.closeOtherTabs': 'Close Other Tabs',
  'tabs.duplicateTab': 'Duplicate Tab',
  'tabs.pinTab': 'Pin Tab',
  'tabs.unpinTab': 'Unpin Tab',
  
  // Proxy
  'proxy.settings': 'Proxy Settings',
  'proxy.noProxy': 'No Proxy',
  'proxy.useProxy': 'Use Proxy',
  'proxy.proxyType': 'Proxy Type',
  'proxy.host': 'Host',
  'proxy.port': 'Port',
  'proxy.username': 'Username',
  'proxy.password': 'Password',
  'proxy.testProxy': 'Test Proxy',
  'proxy.rotateIp': 'Rotate IP',
  'proxy.validateIp': 'Validate IP',
  'proxy.export': 'Export Proxies',
  'proxy.import': 'Import Proxies',
  
  // Settings
  'settings.title': 'Settings',
  'settings.general': 'General',
  'settings.appearance': 'Appearance',
  'settings.privacy': 'Privacy',
  'settings.performance': 'Performance',
  'settings.shortcuts': 'Keyboard Shortcuts',
  'settings.language': 'Language',
  'settings.theme': 'Theme',
  'settings.themeLight': 'Light',
  'settings.themeDark': 'Dark',
  'settings.themeSystem': 'System',
  
  // Session
  'session.save': 'Save Session',
  'session.restore': 'Restore Session',
  'session.export': 'Export Session',
  'session.import': 'Import Session',
  'session.clear': 'Clear Session',
  'session.autoSave': 'Auto-save Session',
  
  // Status
  'status.connected': 'Connected',
  'status.disconnected': 'Disconnected',
  'status.connecting': 'Connecting...',
  'status.proxyActive': 'Proxy Active',
  'status.directConnection': 'Direct Connection',
  
  // Errors
  'error.connectionFailed': 'Connection failed',
  'error.proxyError': 'Proxy error',
  'error.timeout': 'Request timeout',
  'error.invalidUrl': 'Invalid URL',
  'error.unknown': 'Unknown error',
};

/**
 * Spanish translations
 */
const es: TranslationDict = {
  'common.ok': 'Aceptar',
  'common.cancel': 'Cancelar',
  'common.save': 'Guardar',
  'common.delete': 'Eliminar',
  'common.edit': 'Editar',
  'common.close': 'Cerrar',
  'common.search': 'Buscar',
  'common.loading': 'Cargando...',
  'common.error': 'Error',
  'common.success': 'Éxito',
  'common.warning': 'Advertencia',
  'common.confirm': 'Confirmar',
  'common.yes': 'Sí',
  'common.no': 'No',
  
  'nav.back': 'Atrás',
  'nav.forward': 'Adelante',
  'nav.reload': 'Recargar',
  'nav.stop': 'Detener',
  'nav.home': 'Inicio',
  
  'tabs.newTab': 'Nueva pestaña',
  'tabs.closeTab': 'Cerrar pestaña',
  'tabs.closeOtherTabs': 'Cerrar otras pestañas',
  'tabs.duplicateTab': 'Duplicar pestaña',
  'tabs.pinTab': 'Fijar pestaña',
  'tabs.unpinTab': 'Desfijar pestaña',
  
  'proxy.settings': 'Configuración de proxy',
  'proxy.noProxy': 'Sin proxy',
  'proxy.useProxy': 'Usar proxy',
  'proxy.proxyType': 'Tipo de proxy',
  'proxy.host': 'Host',
  'proxy.port': 'Puerto',
  'proxy.username': 'Usuario',
  'proxy.password': 'Contraseña',
  'proxy.testProxy': 'Probar proxy',
  'proxy.rotateIp': 'Rotar IP',
  'proxy.validateIp': 'Validar IP',
  'proxy.export': 'Exportar proxies',
  'proxy.import': 'Importar proxies',
  
  'settings.title': 'Configuración',
  'settings.general': 'General',
  'settings.appearance': 'Apariencia',
  'settings.privacy': 'Privacidad',
  'settings.performance': 'Rendimiento',
  'settings.shortcuts': 'Atajos de teclado',
  'settings.language': 'Idioma',
  'settings.theme': 'Tema',
  'settings.themeLight': 'Claro',
  'settings.themeDark': 'Oscuro',
  'settings.themeSystem': 'Sistema',
  
  'session.save': 'Guardar sesión',
  'session.restore': 'Restaurar sesión',
  'session.export': 'Exportar sesión',
  'session.import': 'Importar sesión',
  'session.clear': 'Limpiar sesión',
  'session.autoSave': 'Autoguardar sesión',
  
  'status.connected': 'Conectado',
  'status.disconnected': 'Desconectado',
  'status.connecting': 'Conectando...',
  'status.proxyActive': 'Proxy activo',
  'status.directConnection': 'Conexión directa',
  
  'error.connectionFailed': 'Error de conexión',
  'error.proxyError': 'Error de proxy',
  'error.timeout': 'Tiempo de espera agotado',
  'error.invalidUrl': 'URL inválida',
  'error.unknown': 'Error desconocido',
};

/**
 * All translations
 */
const translations: Record<SupportedLanguage, TranslationDict> = {
  en,
  es,
  fr: en, // Fallback to English for now
  de: en,
  zh: en,
  ja: en,
  ko: en,
  pt: en,
  ru: en,
  ar: en,
};

/**
 * Current language store (persisted)
 */
export const currentLanguage = persist(
  writable<SupportedLanguage>('en'),
  'virtual-ip-browser-language'
);

/**
 * Current language info derived store
 */
export const currentLanguageInfo = derived(
  currentLanguage,
  $lang => languages[$lang]
);

/**
 * Is RTL derived store
 */
export const isRtl = derived(
  currentLanguageInfo,
  $info => $info.rtl
);

/**
 * Detect browser language
 */
export function detectBrowserLanguage(): SupportedLanguage {
  if (typeof navigator === 'undefined') return 'en';
  
  const browserLang = navigator.language.split('-')[0] as SupportedLanguage;
  return languages[browserLang] ? browserLang : 'en';
}

/**
 * Set current language
 */
export function setLanguage(lang: SupportedLanguage): void {
  currentLanguage.set(lang);
  
  // Update document direction for RTL languages
  if (typeof document !== 'undefined') {
    document.documentElement.dir = languages[lang].rtl ? 'rtl' : 'ltr';
    document.documentElement.lang = lang;
  }
}

/**
 * Get translation for a key
 */
export function t(key: string, params?: Record<string, string | number>): string {
  const lang = get(currentLanguage);
  const dict = translations[lang] || translations.en;
  
  let text = (dict[key] as string) || (translations.en[key] as string) || key;
  
  // Replace parameters
  if (params) {
    Object.entries(params).forEach(([paramKey, value]) => {
      text = text.replace(new RegExp(`{${paramKey}}`, 'g'), String(value));
    });
  }
  
  return text;
}

/**
 * Create a reactive translation function
 */
export function createTranslator() {
  return derived(currentLanguage, $lang => {
    return (key: string, params?: Record<string, string | number>): string => {
      const dict = translations[$lang] || translations.en;
      let text = (dict[key] as string) || (translations.en[key] as string) || key;
      
      if (params) {
        Object.entries(params).forEach(([paramKey, value]) => {
          text = text.replace(new RegExp(`{${paramKey}}`, 'g'), String(value));
        });
      }
      
      return text;
    };
  });
}

/**
 * Format number according to locale
 */
export function formatNumber(num: number, lang?: SupportedLanguage): string {
  const l = lang || get(currentLanguage);
  const info = languages[l];
  
  const parts = num.toString().split('.');
  parts[0] = parts[0].replace(/\B(?=(\d{3})+(?!\d))/g, info.numberFormat.thousand);
  
  return parts.join(info.numberFormat.decimal);
}

/**
 * Format date according to locale
 */
export function formatDate(date: Date, lang?: SupportedLanguage): string {
  const l = lang || get(currentLanguage);
  return new Intl.DateTimeFormat(l).format(date);
}

/**
 * Get all available languages
 */
export function getAvailableLanguages(): LanguageInfo[] {
  return Object.values(languages);
}

// Initialize language on load
if (typeof window !== 'undefined') {
  const savedLang = get(currentLanguage);
  setLanguage(savedLang);
}

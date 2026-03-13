export const config = {
  apiBaseUrl: import.meta.env.VITE_API_BASE_URL || 'http://localhost:8080',
  apiMode: (import.meta.env.VITE_API_MODE || 'hybrid').toLowerCase(),
}

export function isMockMode() {
  return config.apiMode === 'mock'
}

export function isHybridMode() {
  return config.apiMode === 'hybrid'
}

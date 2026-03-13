import { createContext, useCallback, useContext, useEffect, useMemo, useState } from 'react'
import { api } from '../lib/api'

const AuthContext = createContext(null)
const STORAGE_KEY = 'project-registration-auth'

export function AuthProvider({ children }) {
  const [session, setSession] = useState(() => {
    const raw = localStorage.getItem(STORAGE_KEY)
    if (!raw) return { token: '', user: null }
    try {
      return JSON.parse(raw)
    } catch {
      return { token: '', user: null }
    }
  })
  const [initializing, setInitializing] = useState(true)

  useEffect(() => {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(session))
  }, [session])

  useEffect(() => {
    let active = true

    async function hydrate() {
      if (!session.token || !session.user) {
        setInitializing(false)
        return
      }

      try {
        const user = await api.getMe(session)
        if (active && user) {
          setSession((previous) => ({ ...previous, user }))
        }
      } catch {
        // keep last successful session snapshot when backend route is not ready
      } finally {
        if (active) setInitializing(false)
      }
    }

    hydrate()
    return () => {
      active = false
    }
  }, [])

  const login = useCallback(async (credentials) => {
    const nextSession = await api.login(credentials)
    setSession(nextSession)
    return nextSession
  }, [])

  const register = useCallback(async (payload) => {
    const nextSession = await api.register(payload)
    setSession(nextSession)
    return nextSession
  }, [])

  const logout = useCallback(async () => {
    try {
      if (session.token) {
        await api.logout(session)
      }
    } catch {
      // ignore logout API failures and clear local session anyway
    } finally {
      setSession({ token: '', user: null })
    }
  }, [session])

  const refreshNotifications = useCallback(async () => {
    if (!session.user) return []
    return api.listNotifications(session, session.user.id)
  }, [session])

  const value = useMemo(
    () => ({
      token: session.token,
      user: session.user,
      initializing,
      isAuthenticated: Boolean(session.token),
      login,
      register,
      logout,
      refreshNotifications,
    }),
    [session, initializing, login, register, logout, refreshNotifications],
  )

  return <AuthContext.Provider value={value}>{children}</AuthContext.Provider>
}

export function useAuth() {
  const context = useContext(AuthContext)
  if (!context) {
    throw new Error('useAuth must be used within AuthProvider')
  }
  return context
}

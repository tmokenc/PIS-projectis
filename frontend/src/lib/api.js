import { config, isHybridMode, isMockMode } from './config'
import { mockApi } from './mockApi'

function makeHeaders(session, extra = {}) {
  const headers = { 'Content-Type': 'application/json', ...extra }
  if (session?.token) {
    headers.Authorization = `Bearer ${session.token}`
  }
  return headers
}

async function request(path, options = {}) {
  const response = await fetch(`${config.apiBaseUrl}${path}`, options)
  if (!response.ok) {
    let message = `Request failed with status ${response.status}`
    try {
      const data = await response.json()
      message = data.message || data.error || message
    } catch {
      // ignore json parse failure
    }
    throw new Error(message)
  }

  if (response.status === 204) {
    return null
  }

  return response.json()
}

async function tryLive(fn, fallback) {
  if (isMockMode()) {
    return fallback()
  }

  try {
    return await fn()
  } catch (error) {
    if (isHybridMode()) {
      return fallback(error)
    }
    throw error
  }
}

function normalizeAuthResponse(data) {
  return {
    token: data.access_token || data.token || '',
    user: data.user
      ? {
          id: data.user.id,
          firstname: data.user.firstname,
          lastname: data.user.lastname,
          email: data.user.email,
          role: String(data.user.role || '').toLowerCase().replace('user_role_', ''),
        }
      : null,
  }
}

export const api = {
  async login(credentials) {
    return tryLive(
      async () => normalizeAuthResponse(await request('/api/auth/login', {
        method: 'POST',
        headers: makeHeaders(),
        body: JSON.stringify(credentials),
      })),
      () => mockApi.login(credentials),
    )
  },

  async register(payload) {
    return tryLive(
      async () => normalizeAuthResponse(await request('/api/auth/register', {
        method: 'POST',
        headers: makeHeaders(),
        body: JSON.stringify(payload),
      })),
      () => mockApi.register(payload),
    )
  },

  async getMe(session) {
    return tryLive(
      async () => {
        const live = await request('/api/auth/me', {
          method: 'GET',
          headers: makeHeaders(session),
        })
        return normalizeAuthResponse({ user: live }).user
      },
      async () => {
        if (session.user?.id) {
          return mockApi.getMe(session)
        }
        return session.user
      },
    )
  },

  async logout(session) {
    return tryLive(
      async () => request('/api/auth/logout', {
        method: 'POST',
        headers: makeHeaders(session),
      }),
      () => mockApi.logout(session),
    )
  },

  async listSubjects(session) {
    return tryLive(
      async () => request('/api/subjects', {
        method: 'GET',
        headers: makeHeaders(session),
      }),
      () => mockApi.listSubjects(session),
    )
  },

  async createSubject(session, payload) {
    return tryLive(
      async () => request('/api/subjects', {
        method: 'POST',
        headers: makeHeaders(session),
        body: JSON.stringify(payload),
      }),
      () => mockApi.createSubject(session, payload),
    )
  },

  async updateSubject(session, subjectId, payload) {
    return tryLive(
      async () => request(`/api/subjects/${subjectId}`, {
        method: 'PUT',
        headers: makeHeaders(session),
        body: JSON.stringify(payload),
      }),
      () => mockApi.updateSubject(session, subjectId, payload),
    )
  },

  async deleteSubject(session, subjectId) {
    return tryLive(
      async () => request(`/api/subjects/${subjectId}`, {
        method: 'DELETE',
        headers: makeHeaders(session),
      }),
      () => mockApi.deleteSubject(session, subjectId),
    )
  },

  async registerSubject(session, subjectId) {
    return tryLive(
      async () => request('/api/subjects/register', {
        method: 'POST',
        headers: makeHeaders(session),
        body: JSON.stringify({ subject_id: subjectId }),
      }),
      () => mockApi.registerSubject(session, subjectId),
    )
  },

  async listProjects(session) {
    return tryLive(
      async () => request('/api/projects', {
        method: 'GET',
        headers: makeHeaders(session),
      }),
      () => mockApi.listProjects(session),
    )
  },

  async getProject(session, projectId) {
    return tryLive(
      async () => request(`/api/projects/${projectId}`, {
        method: 'GET',
        headers: makeHeaders(session),
      }),
      () => mockApi.getProject(projectId),
    )
  },

  async createProject(session, payload) {
    return tryLive(
      async () => request('/api/projects', {
        method: 'POST',
        headers: makeHeaders(session),
        body: JSON.stringify(payload),
      }),
      () => mockApi.createProject(session, payload),
    )
  },

  async registerTeam(session, projectId) {
    return tryLive(
      async () => request('/api/projects/register', {
        method: 'POST',
        headers: makeHeaders(session),
        body: JSON.stringify({ project_id: projectId }),
      }),
      () => mockApi.registerTeam(session, projectId),
    )
  },

  async addTeamMember(session, teamId, studentId) {
    return tryLive(
      async () => request('/api/teams/members', {
        method: 'POST',
        headers: makeHeaders(session),
        body: JSON.stringify({ team_id: teamId, student_id: studentId }),
      }),
      () => mockApi.addTeamMember(session, teamId, studentId),
    )
  },

  async listTeamsByProject(session, projectId) {
    return tryLive(
      async () => request(`/api/projects/${projectId}/teams`, {
        method: 'GET',
        headers: makeHeaders(session),
      }),
      () => mockApi.listTeamsByProject(projectId),
    )
  },

  async listNotifications(session, _userId) {
    // don't know why chat insert userId here, while the API does not expect it
    // leave it as comment for now, in case we need it in the future
    // const query = userId ? `?user_id=${encodeURIComponent(userId)}` : ''
    const query = ''
    return tryLive(
      async () => request(`/api/notifications${query}`, {
        method: 'GET',
        headers: makeHeaders(session),
      }),
      () => mockApi.listNotifications(session, userId),
    )
  },

  async createNotification(session, payload) {
    return tryLive(
      async () => request('/api/notifications', {
        method: 'POST',
        headers: makeHeaders(session),
        body: JSON.stringify(payload),
      }),
      () => mockApi.createNotification(session, payload),
    )
  },

  async markNotificationRead(session, notificationId) {
    return tryLive(
      async () => request(`/api/notifications/${notificationId}/read`, {
        method: 'POST',
        headers: makeHeaders(session),
      }),
      () => mockApi.markNotificationRead(session, notificationId),
    )
  },

  async listUsers(session) {
    return tryLive(
      async () => request('/api/admin/users', {
        method: 'GET',
        headers: makeHeaders(session),
      }),
      () => mockApi.listUsers(session),
    )
  },

  async getDashboardSummary(session) {
    return tryLive(
      async () => request('/api/dashboard', {
        method: 'GET',
        headers: makeHeaders(session),
      }),
      () => mockApi.getDashboardSummary(session),
    )
  },
}

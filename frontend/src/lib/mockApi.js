const now = new Date()
const day = 24 * 60 * 60 * 1000

let users = [
  {
    id: 'user-student-1',
    firstname: 'An',
    lastname: 'Nguyen',
    email: 'student@example.com',
    password: 'student123',
    role: 'student',
  },
  {
    id: 'user-teacher-1',
    firstname: 'Marek',
    lastname: 'Novak',
    email: 'teacher@example.com',
    password: 'teacher123',
    role: 'teacher',
  },
  {
    id: 'user-admin-1',
    firstname: 'Eva',
    lastname: 'Svobodova',
    email: 'admin@example.com',
    password: 'admin123',
    role: 'admin',
  },
]

let subjects = [
  {
    id: 'subject-1',
    name: 'Secure Software Systems',
    description: 'Focuses on secure coding, threat modeling, and software assurance.',
    abbreviation: 'SSS',
  },
  {
    id: 'subject-2',
    name: 'Distributed Applications',
    description: 'Covers service design, microservices, messaging, and observability.',
    abbreviation: 'DIA',
  },
  {
    id: 'subject-3',
    name: 'Applied Cryptography',
    description: 'Practical symmetric and asymmetric cryptography used in real systems.',
    abbreviation: 'ACR',
  },
]

let projects = [
  {
    id: 'project-1',
    title: 'Microservice Router with Distributed Tracing',
    description: 'Create an API gateway and integrate OpenTelemetry-based tracing.',
    teacher_id: 'user-teacher-1',
    max_students_per_team: 3,
    start_date: iso(now),
    end_date: iso(new Date(now.getTime() + 60 * day)),
    subject_id: 'subject-2',
  },
  {
    id: 'project-2',
    title: 'Authentication Service',
    description: 'Implement auth with HS256 JWT signing, validation, and refresh strategy.',
    teacher_id: 'user-teacher-1',
    max_students_per_team: 2,
    start_date: iso(new Date(now.getTime() + 3 * day)),
    end_date: iso(new Date(now.getTime() + 80 * day)),
    subject_id: 'subject-3',
  },
  {
    id: 'project-3',
    title: 'Notification Delivery Service',
    description: 'Provide read state, deadlines, and event-driven alerts for students.',
    teacher_id: 'user-teacher-1',
    max_students_per_team: 4,
    start_date: iso(new Date(now.getTime() + 5 * day)),
    end_date: iso(new Date(now.getTime() + 70 * day)),
    subject_id: 'subject-1',
  },
]

let teams = [
  {
    id: 'team-1',
    project_id: 'project-1',
    name: 'Tracefinders',
    leader_student_id: 'user-student-1',
    student_ids: ['user-student-1'],
  },
]

let notifications = [
  {
    id: 'notification-1',
    user_id: 'user-student-1',
    message: 'Registration for Distributed Applications is open.',
    date: iso(new Date(now.getTime() - 2 * day)),
    read: false,
  },
  {
    id: 'notification-2',
    user_id: 'user-student-1',
    message: 'Project proposal deadline is in 7 days.',
    date: iso(new Date(now.getTime() - 1 * day)),
    read: false,
  },
  {
    id: 'notification-3',
    user_id: 'user-teacher-1',
    message: 'A new student registered interest in your project.',
    date: iso(new Date(now.getTime() - 6 * 60 * 60 * 1000)),
    read: false,
  },
]

let subjectRegistrations = {
  'user-student-1': ['subject-2'],
}

function iso(value) {
  return value.toISOString()
}

function delay(result, ms = 250) {
  return new Promise((resolve) => {
    setTimeout(() => resolve(structuredClone(result)), ms)
  })
}

function generateId(prefix) {
  return `${prefix}-${Math.random().toString(36).slice(2, 10)}`
}

function stripPassword(user) {
  const { password, ...rest } = user
  return rest
}

function authSession(user) {
  return {
    token: `mock-token-${user.id}`,
    user: stripPassword(user),
  }
}

function currentUserFromSession(session) {
  return users.find((user) => user.id === session.user?.id) || null
}

export const mockApi = {
  async login({ email, password }) {
    const user = users.find((item) => item.email === email && item.password === password)
    if (!user) {
      throw new Error('Invalid email or password')
    }
    return delay(authSession(user))
  },

  async register(payload) {
    const exists = users.some((user) => user.email === payload.email)
    if (exists) throw new Error('Email already registered')

    const user = {
      id: generateId('user'),
      firstname: payload.firstname,
      lastname: payload.lastname,
      email: payload.email,
      password: payload.password,
      role: payload.role,
    }

    users = [user, ...users]
    notifications = [
      {
        id: generateId('notification'),
        user_id: user.id,
        message: 'Welcome to the project registration system.',
        date: iso(new Date()),
        read: false,
      },
      ...notifications,
    ]

    return delay(authSession(user))
  },

  async getMe(session) {
    const user = currentUserFromSession(session)
    if (!user) throw new Error('Unauthorized')
    return delay(stripPassword(user))
  },

  async logout() {
    return delay({ success: true })
  },

  async listSubjects() {
    return delay(subjects)
  },

  async createSubject(session, payload) {
    if (session.user?.role !== 'admin') throw new Error('Forbidden')
    const subject = { id: generateId('subject'), ...payload }
    subjects = [subject, ...subjects]
    return delay(subject)
  },

  async updateSubject(session, subjectId, payload) {
    if (session.user?.role !== 'admin') throw new Error('Forbidden')
    subjects = subjects.map((subject) =>
      subject.id === subjectId ? { ...subject, ...payload } : subject,
    )
    return delay(subjects.find((subject) => subject.id === subjectId))
  },

  async deleteSubject(session, subjectId) {
    if (session.user?.role !== 'admin') throw new Error('Forbidden')
    subjects = subjects.filter((subject) => subject.id !== subjectId)
    return delay({ success: true })
  },

  async registerSubject(session, subjectId) {
    if (session.user?.role !== 'student') throw new Error('Only students can register to subjects')
    const current = new Set(subjectRegistrations[session.user.id] || [])
    current.add(subjectId)
    subjectRegistrations[session.user.id] = [...current]
    notifications = [
      {
        id: generateId('notification'),
        user_id: session.user.id,
        message: `You registered to subject ${subjects.find((s) => s.id === subjectId)?.name || subjectId}.`,
        date: iso(new Date()),
        read: false,
      },
      ...notifications,
    ]
    return delay({ success: true })
  },

  async listProjects() {
    return delay(projects)
  },

  async getProject(projectId) {
    const project = projects.find((item) => item.id === projectId)
    if (!project) throw new Error('Project not found')
    return delay(project)
  },

  async createProject(session, payload) {
    if (!['teacher', 'admin'].includes(session.user?.role)) throw new Error('Forbidden')
    const project = {
      id: generateId('project'),
      title: payload.title,
      description: payload.description,
      teacher_id: payload.teacher_id || session.user.id,
      max_students_per_team: Number(payload.max_students_per_team),
      start_date: payload.start_date,
      end_date: payload.end_date,
      subject_id: payload.subject_id,
    }
    projects = [project, ...projects]
    return delay(project)
  },

  async registerTeam(session, projectId) {
    if (session.user?.role !== 'student') throw new Error('Only students can create teams')
    const team = {
      id: generateId('team'),
      project_id: projectId,
      name: `${session.user.firstname}'s Team`,
      leader_student_id: session.user.id,
      student_ids: [session.user.id],
    }
    teams = [team, ...teams]
    return delay(team)
  },

  async addTeamMember(session, teamId, studentId) {
    const team = teams.find((item) => item.id === teamId)
    if (!team) throw new Error('Team not found')
    if (team.leader_student_id !== session.user?.id && session.user?.role !== 'teacher' && session.user?.role !== 'admin') {
      throw new Error('Forbidden')
    }
    if (!team.student_ids.includes(studentId)) {
      team.student_ids.push(studentId)
    }
    return delay(team)
  },

  async listTeamsByProject(projectId) {
    return delay(teams.filter((team) => team.project_id === projectId))
  },

  async listNotifications(session, userId) {
    const effectiveUserId = userId || session.user?.id
    return delay(notifications.filter((item) => item.user_id === effectiveUserId))
  },

  async createNotification(session, payload) {
    if (!['teacher', 'admin'].includes(session.user?.role)) throw new Error('Forbidden')
    const notification = {
      id: generateId('notification'),
      user_id: payload.user_id,
      message: payload.message,
      date: iso(new Date()),
      read: false,
    }
    notifications = [notification, ...notifications]
    return delay(notification)
  },

  async markNotificationRead(session, notificationId) {
    notifications = notifications.map((item) =>
      item.id === notificationId ? { ...item, read: true } : item,
    )
    return delay({ success: true })
  },

  async listUsers(session) {
    if (session.user?.role !== 'admin') throw new Error('Forbidden')
    return delay(users.map(stripPassword))
  },

  async getDashboardSummary(session) {
    const registeredSubjects = new Set(subjectRegistrations[session.user?.id] || [])
    const ownTeams = teams.filter((team) => team.student_ids.includes(session.user?.id))
    const ownProjects = projects.filter((project) => project.teacher_id === session.user?.id)
    return delay({
      subjects: subjects.length,
      registeredSubjects: registeredSubjects.size,
      projects: projects.length,
      ownProjects: ownProjects.length,
      teams: ownTeams.length,
      unreadNotifications: notifications.filter(
        (item) => item.user_id === session.user?.id && !item.read,
      ).length,
    })
  },
}

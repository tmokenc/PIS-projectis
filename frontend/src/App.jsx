import { Navigate, Route, Routes } from 'react-router'
import AppShell from './components/AppShell'
import ProtectedRoute from './components/ProtectedRoute'
import LoginPage from './pages/auth/LoginPage'
import RegisterPage from './pages/auth/RegisterPage'
import DashboardPage from './pages/DashboardPage'
import SubjectsPage from './pages/SubjectsPage'
import ProjectsPage from './pages/ProjectsPage'
import NotificationsPage from './pages/NotificationsPage'
import AdminUsersPage from './pages/AdminUsersPage'
import NotFoundPage from './pages/NotFoundPage'

export default function App() {
  return (
    <Routes>
      <Route path="/login" element={<LoginPage />} />
      <Route path="/register" element={<RegisterPage />} />
      <Route
        path="/"
        element={
          <ProtectedRoute>
            <AppShell />
          </ProtectedRoute>
        }
      >
        <Route index element={<Navigate to="/dashboard" replace />} />
        <Route path="dashboard" element={<DashboardPage />} />
        <Route path="subjects" element={<SubjectsPage />} />
        <Route path="projects" element={<ProjectsPage />} />
        <Route path="notifications" element={<NotificationsPage />} />
        <Route
          path="admin/users"
          element={
            <ProtectedRoute roles={["admin"]}>
              <AdminUsersPage />
            </ProtectedRoute>
          }
        />
      </Route>
      <Route path="*" element={<NotFoundPage />} />
    </Routes>
  )
}

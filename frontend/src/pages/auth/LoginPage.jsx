import { Alert, Box, Button, Link, Stack, TextField } from '@mui/material'
import { useState } from 'react'
import { Link as RouterLink, useLocation, useNavigate } from 'react-router'
import AuthLayout from './AuthLayout'
import { useAuth } from '../../contexts/AuthContext'

export default function LoginPage() {
  const navigate = useNavigate()
  const location = useLocation()
  const { login } = useAuth()
  const [form, setForm] = useState({ email: '', password: '' })
  const [error, setError] = useState('')
  const [submitting, setSubmitting] = useState(false)

  const from = location.state?.from?.pathname || '/dashboard'

  async function handleSubmit(event) {
    event.preventDefault()
    setError('')
    setSubmitting(true)

    try {
      await login(form)
      navigate(from, { replace: true })
    } catch (err) {
      setError(err.message || 'Login failed')
    } finally {
      setSubmitting(false)
    }
  }

  return (
    <AuthLayout
      title="Welcome back"
      subtitle="Sign in to continue to your role-specific workspace."
    >
      <Box component="form" onSubmit={handleSubmit}>
        <Stack spacing={2.5}>
          {error && <Alert severity="error">{error}</Alert>}
          <TextField
            label="Email"
            type="email"
            fullWidth
            value={form.email}
            onChange={(event) => setForm((prev) => ({ ...prev, email: event.target.value }))}
          />
          <TextField
            label="Password"
            type="password"
            fullWidth
            value={form.password}
            onChange={(event) => setForm((prev) => ({ ...prev, password: event.target.value }))}
          />
          <Button size="large" type="submit" variant="contained" disabled={submitting}>
            {submitting ? 'Signing in...' : 'Sign in'}
          </Button>
          <Link component={RouterLink} to="/register" underline="hover">
            Need an account? Register here.
          </Link>
          <Alert severity="info">
            Demo accounts in mock mode: student@example.com / student123, teacher@example.com / teacher123, admin@example.com / admin123.
          </Alert>
        </Stack>
      </Box>
    </AuthLayout>
  )
}

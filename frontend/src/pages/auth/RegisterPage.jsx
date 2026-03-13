import {
  Alert,
  Box,
  Button,
  Link,
  MenuItem,
  Stack,
  TextField,
} from '@mui/material'
import { useState } from 'react'
import { Link as RouterLink, useNavigate } from 'react-router'
import AuthLayout from './AuthLayout'
import { useAuth } from '../../contexts/AuthContext'

const roleOptions = [
  { value: 'student', label: 'Student' },
  { value: 'teacher', label: 'Teacher' },
  { value: 'admin', label: 'Admin' },
]

export default function RegisterPage() {
  const navigate = useNavigate()
  const { register } = useAuth()
  const [form, setForm] = useState({
    firstname: '',
    lastname: '',
    email: '',
    password: '',
    role: 'student',
  })
  const [error, setError] = useState('')
  const [submitting, setSubmitting] = useState(false)

  async function handleSubmit(event) {
    event.preventDefault()
    setError('')
    setSubmitting(true)

    try {
      await register(form)
      navigate('/dashboard', { replace: true })
    } catch (err) {
      setError(err.message || 'Registration failed')
    } finally {
      setSubmitting(false)
    }
  }

  return (
    <AuthLayout
      title="Create an account"
      subtitle="Register as a student, teacher, or admin to access the platform."
    >
      <Box component="form" onSubmit={handleSubmit}>
        <Stack spacing={2.5}>
          {error && <Alert severity="error">{error}</Alert>}
          <Stack direction={{ xs: 'column', sm: 'row' }} spacing={2}>
            <TextField
              label="First name"
              fullWidth
              value={form.firstname}
              onChange={(event) => setForm((prev) => ({ ...prev, firstname: event.target.value }))}
            />
            <TextField
              label="Last name"
              fullWidth
              value={form.lastname}
              onChange={(event) => setForm((prev) => ({ ...prev, lastname: event.target.value }))}
            />
          </Stack>
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
          <TextField
            select
            label="Role"
            fullWidth
            value={form.role}
            onChange={(event) => setForm((prev) => ({ ...prev, role: event.target.value }))}
          >
            {roleOptions.map((option) => (
              <MenuItem key={option.value} value={option.value}>
                {option.label}
              </MenuItem>
            ))}
          </TextField>
          <Button size="large" type="submit" variant="contained" disabled={submitting}>
            {submitting ? 'Creating account...' : 'Create account'}
          </Button>
          <Link component={RouterLink} to="/login" underline="hover">
            Already registered? Sign in.
          </Link>
        </Stack>
      </Box>
    </AuthLayout>
  )
}

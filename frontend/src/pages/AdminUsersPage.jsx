import {
  Alert,
  Avatar,
  Box,
  Card,
  CardContent,
  Chip,
  Stack,
  Typography,
} from '@mui/material'
import ManageAccountsRoundedIcon from '@mui/icons-material/ManageAccountsRounded'
import { useEffect, useMemo, useState } from 'react'
import EmptyState from '../components/EmptyState'
import LoadingState from '../components/LoadingState'
import PageHeader from '../components/PageHeader'
import { useAuth } from '../contexts/AuthContext'
import { api } from '../lib/api'

export default function AdminUsersPage() {
  const { token, user } = useAuth()
  const session = useMemo(() => ({ token, user }), [token, user])
  const [users, setUsers] = useState([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState('')

  useEffect(() => {
    let active = true

    async function loadUsers() {
      setError('')
      try {
        const data = await api.listUsers(session)
        if (active) setUsers(data)
      } catch (err) {
        if (active) setError(err.message || 'Failed to load users')
      } finally {
        if (active) setLoading(false)
      }
    }

    loadUsers()
    return () => {
      active = false
    }
  }, [token, user])

  if (loading) return <LoadingState />

  return (
    <>
      <PageHeader
        eyebrow="Administration"
        title="User management"
        subtitle="Review registered users and roles. This page is ready for the backend once user management endpoints are added."
      />

      <Alert severity="info" sx={{ mb: 3 }}>
        In hybrid or mock mode this page can still display data even if the admin user API is not implemented yet.
      </Alert>

      {error && <Alert severity="error" sx={{ mb: 3 }}>{error}</Alert>}

      {!users.length ? (
        <EmptyState title="No users available" description="No user records were returned by the backend or mock layer." />
      ) : (
        <Box
          sx={{
            display: 'grid',
            gridTemplateColumns: { xs: '1fr', md: 'repeat(2, 1fr)', xl: 'repeat(3, 1fr)' },
            gap: 2.5,
          }}
        >
          {users.map((item) => (
            <Card key={item.id}>
              <CardContent>
                <Stack direction="row" spacing={2} alignItems="center">
                  <Avatar sx={{ bgcolor: 'primary.main' }}>
                    <ManageAccountsRoundedIcon />
                  </Avatar>
                  <Stack sx={{ minWidth: 0 }}>
                    <Typography variant="subtitle1" noWrap>
                      {item.firstname} {item.lastname}
                    </Typography>
                    <Typography variant="body2" color="text.secondary" noWrap>
                      {item.email}
                    </Typography>
                  </Stack>
                </Stack>
                <Stack direction="row" spacing={1} sx={{ mt: 2 }}>
                  <Chip label={item.role} color="secondary" />
                  <Chip label={item.id} variant="outlined" />
                </Stack>
              </CardContent>
            </Card>
          ))}
        </Box>
      )}
    </>
  )
}

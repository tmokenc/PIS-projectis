import {
  Alert,
  Box,
  Button,
  Card,
  CardContent,
  Chip,
  Dialog,
  DialogActions,
  DialogContent,
  DialogTitle,
  Stack,
  TextField,
  Typography,
} from '@mui/material'
import AddAlertRoundedIcon from '@mui/icons-material/AddAlertRounded'
import MarkEmailReadRoundedIcon from '@mui/icons-material/MarkEmailReadRounded'
import PageHeader from '../components/PageHeader'
import EmptyState from '../components/EmptyState'
import LoadingState from '../components/LoadingState'
import { useAuth } from '../contexts/AuthContext'
import { api } from '../lib/api'
import { formatDate } from '../utils/date'
import { useEffect, useMemo, useState } from 'react'

export default function NotificationsPage() {
  const { token, user } = useAuth()
  const session = useMemo(() => ({ token, user }), [token, user])
  const [notifications, setNotifications] = useState([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState('')
  const [dialogOpen, setDialogOpen] = useState(false)
  const [form, setForm] = useState({ user_id: '', message: '' })

  async function loadNotifications() {
    setLoading(true)
    setError('')
    try {
      const items = await api.listNotifications(session, user?.id)
      setNotifications(items)
    } catch (err) {
      setError(err.message || 'Failed to load notifications')
    } finally {
      setLoading(false)
    }
  }

  useEffect(() => {
    loadNotifications()
  }, [token, user])

  async function handleMarkRead(notificationId) {
    try {
      await api.markNotificationRead(session, notificationId)
      await loadNotifications()
    } catch (err) {
      setError(err.message || 'Failed to mark notification as read')
    }
  }

  async function handleCreate() {
    try {
      await api.createNotification(session, form)
      setDialogOpen(false)
      setForm({ user_id: '', message: '' })
      await loadNotifications()
    } catch (err) {
      setError(err.message || 'Failed to create notification')
    }
  }

  if (loading) return <LoadingState />

  return (
    <>
      <PageHeader
        eyebrow="Notifications"
        title="Notification centre"
        subtitle="Track system updates and deadlines. Teachers and admins can also send new notifications."
        actions={
          ['teacher', 'admin'].includes(user?.role) ? (
            <Button variant="contained" startIcon={<AddAlertRoundedIcon />} onClick={() => setDialogOpen(true)}>
              Send notification
            </Button>
          ) : null
        }
      />

      {error && <Alert severity="error" sx={{ mb: 3 }}>{error}</Alert>}

      {!notifications.length ? (
        <EmptyState
          title="No notifications"
          description="You are up to date. New project updates and deadline reminders will appear here."
        />
      ) : (
        <Box
          sx={{
            display: 'grid',
            gridTemplateColumns: { xs: '1fr', lg: 'repeat(2, 1fr)' },
            gap: 2.5,
          }}
        >
          {notifications.map((notification) => (
            <Card key={notification.id} sx={{ borderLeft: notification.read ? undefined : '4px solid', borderLeftColor: 'warning.main' }}>
              <CardContent>
                <Stack direction="row" justifyContent="space-between" alignItems="flex-start" spacing={2}>
                  <Stack spacing={1.25}>
                    <Typography variant="body1">{notification.message}</Typography>
                    <Typography variant="body2" color="text.secondary">
                      {formatDate(notification.date)}
                    </Typography>
                  </Stack>
                  <Chip label={notification.read ? 'Read' : 'Unread'} color={notification.read ? 'default' : 'warning'} />
                </Stack>
                {!notification.read && (
                  <Button sx={{ mt: 2 }} size="small" startIcon={<MarkEmailReadRoundedIcon />} onClick={() => handleMarkRead(notification.id)}>
                    Mark as read
                  </Button>
                )}
              </CardContent>
            </Card>
          ))}
        </Box>
      )}

      <Dialog open={dialogOpen} onClose={() => setDialogOpen(false)} fullWidth maxWidth="sm">
        <DialogTitle>Send notification</DialogTitle>
        <DialogContent>
          <Stack spacing={2} sx={{ mt: 1 }}>
            <TextField label="Target user ID" value={form.user_id} onChange={(event) => setForm((prev) => ({ ...prev, user_id: event.target.value }))} helperText="Use a specific user ID. In mock mode try user-student-1." />
            <TextField label="Message" multiline minRows={4} value={form.message} onChange={(event) => setForm((prev) => ({ ...prev, message: event.target.value }))} />
          </Stack>
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setDialogOpen(false)}>Cancel</Button>
          <Button onClick={handleCreate} variant="contained">Send</Button>
        </DialogActions>
      </Dialog>
    </>
  )
}

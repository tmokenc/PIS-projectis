import {
  Alert,
  Box,
  Button,
  Card,
  CardContent,
  Chip,
  List,
  ListItem,
  ListItemText,
  Paper,
  Stack,
  Typography,
} from '@mui/material'
import ClassRoundedIcon from '@mui/icons-material/ClassRounded'
import FolderOpenRoundedIcon from '@mui/icons-material/FolderOpenRounded'
import NotificationsRoundedIcon from '@mui/icons-material/NotificationsRounded'
import GroupsRoundedIcon from '@mui/icons-material/GroupsRounded'
import ManageAccountsRoundedIcon from '@mui/icons-material/ManageAccountsRounded'
import ArrowForwardRoundedIcon from '@mui/icons-material/ArrowForwardRounded'
import { useEffect, useMemo, useState } from 'react'
import { useNavigate } from 'react-router'
import PageHeader from '../components/PageHeader'
import StatCard from '../components/StatCard'
import LoadingState from '../components/LoadingState'
import { useAuth } from '../contexts/AuthContext'
import { api } from '../lib/api'

const studentTasks = [
  'Review available subjects and register early.',
  'Check projects and create a team for your selected topic.',
  'Open notifications to track proposal deadlines.',
]

const teacherTasks = [
  'Review students registering for your projects.',
  'Create or update project offerings for current subjects.',
  'Send notifications when requirements or dates change.',
]

const adminTasks = [
  'Review user accounts and role distribution.',
  'Maintain the subject catalog for the semester.',
  'Track registrations and resolve project capacity issues.',
]

export default function DashboardPage() {
  const navigate = useNavigate()
  const { token, user } = useAuth()
  const [summary, setSummary] = useState(null)
  const [loading, setLoading] = useState(true)

  useEffect(() => {
    let active = true

    async function load() {
      try {
        const data = await api.getDashboardSummary({ token, user })
        if (active) setSummary(data)
      } finally {
        if (active) setLoading(false)
      }
    }

    load()
    return () => {
      active = false
    }
  }, [token, user])

  const tasks = useMemo(() => {
    if (user?.role === 'teacher') return teacherTasks
    if (user?.role === 'admin') return adminTasks
    return studentTasks
  }, [user?.role])

  if (loading) return <LoadingState />

  return (
    <>
      <PageHeader
        eyebrow="Overview"
        title={`Welcome, ${user?.firstname}`}
        subtitle="Track your registrations, projects, teams, and notifications from a single dashboard."
        actions={
          <Button variant="contained" endIcon={<ArrowForwardRoundedIcon />} onClick={() => navigate('/projects')}>
            Open projects
          </Button>
        }
      />

      <Alert severity="info" sx={{ mb: 3 }}>
        This dashboard adapts to your role. Student, teacher, and admin actions are grouped into the same UI for easier navigation.
      </Alert>

      <Box
        sx={{
          display: 'grid',
          gridTemplateColumns: { xs: '1fr', sm: 'repeat(2, 1fr)', xl: 'repeat(4, 1fr)' },
          gap: 2.5,
        }}
      >
        <StatCard icon={<ClassRoundedIcon />} label="Subjects" value={summary?.subjects ?? 0} />
        <StatCard icon={<FolderOpenRoundedIcon />} label="Projects" value={summary?.projects ?? 0} color="secondary.main" />
        <StatCard icon={<GroupsRoundedIcon />} label="Teams" value={summary?.teams ?? 0} color="success.main" />
        <StatCard icon={<NotificationsRoundedIcon />} label="Unread notifications" value={summary?.unreadNotifications ?? 0} color="warning.main" />
      </Box>

      <Box
        sx={{
          display: 'grid',
          gridTemplateColumns: { xs: '1fr', lg: 'minmax(0, 2fr) minmax(320px, 1fr)' },
          gap: 2.5,
          mt: 2.5,
        }}
      >
        <Card>
          <CardContent>
            <Stack direction={{ xs: 'column', md: 'row' }} spacing={3} alignItems="flex-start">
              <Stack spacing={1} sx={{ flex: 1 }}>
                <Typography variant="h6">Role summary</Typography>
                <Typography variant="body2" color="text.secondary">
                  The current session is configured for the <strong>{user?.role}</strong> workflow.
                </Typography>
                <Stack direction="row" spacing={1} flexWrap="wrap" useFlexGap>
                  <Chip label={`Registered subjects: ${summary?.registeredSubjects ?? 0}`} color="primary" variant="outlined" />
                  <Chip label={`Owned projects: ${summary?.ownProjects ?? 0}`} color="secondary" variant="outlined" />
                </Stack>
              </Stack>
              <Paper sx={{ p: 2, bgcolor: 'background.default', minWidth: { md: 240 } }}>
                <Typography variant="subtitle2" color="text.secondary">
                  Current role
                </Typography>
                <Typography variant="h5" sx={{ mt: 1, textTransform: 'capitalize' }}>
                  {user?.role}
                </Typography>
              </Paper>
            </Stack>
          </CardContent>
        </Card>

        <Card>
          <CardContent>
            <Stack direction="row" spacing={1} alignItems="center" sx={{ mb: 1.5 }}>
              <ManageAccountsRoundedIcon color="primary" />
              <Typography variant="h6">Next actions</Typography>
            </Stack>
            <List disablePadding>
              {tasks.map((task) => (
                <ListItem key={task} disableGutters>
                  <ListItemText primary={task} />
                </ListItem>
              ))}
            </List>
          </CardContent>
        </Card>
      </Box>
    </>
  )
}

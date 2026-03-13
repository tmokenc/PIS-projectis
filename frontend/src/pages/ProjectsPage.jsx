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
  Divider,
  Stack,
  TextField,
  Typography,
} from '@mui/material'
import AddRoundedIcon from '@mui/icons-material/AddRounded'
import GroupsRoundedIcon from '@mui/icons-material/GroupsRounded'
import EventRoundedIcon from '@mui/icons-material/EventRounded'
import LaunchRoundedIcon from '@mui/icons-material/LaunchRounded'
import PageHeader from '../components/PageHeader'
import LoadingState from '../components/LoadingState'
import EmptyState from '../components/EmptyState'
import { useAuth } from '../contexts/AuthContext'
import { api } from '../lib/api'
import { formatDateOnly } from '../utils/date'
import { useEffect, useMemo, useState } from 'react'

const initialProjectForm = {
  title: '',
  description: '',
  subject_id: '',
  max_students_per_team: 3,
  start_date: '',
  end_date: '',
}

export default function ProjectsPage() {
  const { token, user } = useAuth()
  const session = useMemo(() => ({ token, user }), [token, user])
  const [projects, setProjects] = useState([])
  const [subjects, setSubjects] = useState([])
  const [teams, setTeams] = useState([])
  const [selectedProject, setSelectedProject] = useState(null)
  const [createDialogOpen, setCreateDialogOpen] = useState(false)
  const [memberDialogOpen, setMemberDialogOpen] = useState(false)
  const [form, setForm] = useState(initialProjectForm)
  const [memberStudentId, setMemberStudentId] = useState('')
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState('')

  async function loadData() {
    setLoading(true)
    setError('')
    try {
      const [projectData, subjectData] = await Promise.all([
        api.listProjects(session),
        api.listSubjects(session),
      ])
      setProjects(Array.isArray(projectData) ? projectData : projectData.projects || [])
      setSubjects(Array.isArray(subjectData) ? subjectData : subjectData.subjects || [])
    } catch (err) {
      setError(err.message || 'Failed to load projects')
    } finally {
      setLoading(false)
    }
  }

  useEffect(() => {
    loadData()
  }, [token, user])

  async function openProject(project) {
    setSelectedProject(project)
    try {
      const projectTeams = await api.listTeamsByProject(session, project.id)
      setTeams(Array.isArray(projectTeams) ? projectTeams : projectTeams.teams || [])
    } catch {
      setTeams([])
    }
  }

  async function handleRegisterTeam(projectId) {
    try {
      await api.registerTeam(session, projectId)
      const project = projects.find((item) => item.id === projectId)
      if (project) {
        await openProject(project)
      }
    } catch (err) {
      setError(err.message || 'Failed to create team')
    }
  }

  async function handleCreateProject() {
    try {
      await api.createProject(session, form)
      setCreateDialogOpen(false)
      setForm(initialProjectForm)
      await loadData()
    } catch (err) {
      setError(err.message || 'Failed to create project')
    }
  }

  async function handleAddMember() {
    if (!teams[0]?.id) return
    try {
      await api.addTeamMember(session, teams[0].id, memberStudentId)
      setMemberDialogOpen(false)
      setMemberStudentId('')
      if (selectedProject) {
        await openProject(selectedProject)
      }
    } catch (err) {
      setError(err.message || 'Failed to add team member')
    }
  }

  if (loading) return <LoadingState />

  return (
    <>
      <PageHeader
        eyebrow="Projects"
        title="Project marketplace"
        subtitle="Students can join projects and create teams, while teachers and admins can manage project offerings."
        actions={
          ['teacher', 'admin'].includes(user?.role) ? (
            <Button variant="contained" startIcon={<AddRoundedIcon />} onClick={() => setCreateDialogOpen(true)}>
              Create project
            </Button>
          ) : null
        }
      />

      {error && <Alert severity="error" sx={{ mb: 3 }}>{error}</Alert>}

      {!projects.length ? (
        <EmptyState
          title="No projects available"
          description="Create a project to start the registration cycle or verify that the project service is reachable."
        />
      ) : (
        <Box
          sx={{
            display: 'grid',
            gridTemplateColumns: { xs: '1fr', md: 'repeat(2, 1fr)', xl: 'repeat(3, 1fr)' },
            gap: 2.5,
          }}
        >
          {projects.map((project) => {
            const subject = subjects.find((item) => item.id === project.subject_id)
            return (
              <Card key={project.id} sx={{ height: '100%' }}>
                <CardContent sx={{ display: 'flex', flexDirection: 'column', gap: 2.5, height: '100%' }}>
                  <Stack direction="row" justifyContent="space-between" spacing={1.5}>
                    <Box>
                      <Typography variant="h6">{project.title}</Typography>
                      <Typography variant="body2" color="text.secondary">
                        Subject: {subject?.name || project.subject_id}
                      </Typography>
                    </Box>
                    <Chip icon={<GroupsRoundedIcon />} label={`${project.max_students_per_team} / team`} color="secondary" />
                  </Stack>
                  <Typography variant="body2" color="text.secondary" sx={{ flexGrow: 1 }}>
                    {project.description}
                  </Typography>
                  <Stack direction="row" spacing={2} flexWrap="wrap" useFlexGap>
                    <Chip icon={<EventRoundedIcon />} label={`Start ${formatDateOnly(project.start_date)}`} variant="outlined" />
                    <Chip icon={<EventRoundedIcon />} label={`End ${formatDateOnly(project.end_date)}`} variant="outlined" />
                  </Stack>
                  <Stack direction="row" spacing={1} flexWrap="wrap" useFlexGap>
                    <Button variant="outlined" startIcon={<LaunchRoundedIcon />} onClick={() => openProject(project)}>
                      View details
                    </Button>
                    {user?.role === 'student' && (
                      <Button variant="contained" onClick={() => handleRegisterTeam(project.id)}>
                        Create team
                      </Button>
                    )}
                  </Stack>
                </CardContent>
              </Card>
            )
          })}
        </Box>
      )}

      <Dialog open={Boolean(selectedProject)} onClose={() => setSelectedProject(null)} fullWidth maxWidth="md">
        <DialogTitle>{selectedProject?.title}</DialogTitle>
        <DialogContent>
          {selectedProject && (
            <Stack spacing={2.5} sx={{ mt: 1 }}>
              <Typography variant="body1">{selectedProject.description}</Typography>
              <Stack direction={{ xs: 'column', sm: 'row' }} spacing={1.5}>
                <Chip label={`Start: ${formatDateOnly(selectedProject.start_date)}`} />
                <Chip label={`End: ${formatDateOnly(selectedProject.end_date)}`} />
                <Chip label={`Max students/team: ${selectedProject.max_students_per_team}`} color="secondary" />
              </Stack>
              <Divider />
              <Box>
                <Typography variant="h6" sx={{ mb: 1.5 }}>Teams</Typography>
                {!teams.length ? (
                  <Typography variant="body2" color="text.secondary">
                    No teams registered for this project yet.
                  </Typography>
                ) : (
                  <Stack spacing={1.5}>
                    {teams.map((team) => (
                      <Card key={team.id} variant="outlined">
                        <CardContent>
                          <Typography variant="subtitle1">{team.name}</Typography>
                          <Typography variant="body2" color="text.secondary">
                            Leader: {team.leader_student_id}
                          </Typography>
                          <Typography variant="body2" color="text.secondary">
                            Members: {team.student_ids.join(', ')}
                          </Typography>
                        </CardContent>
                      </Card>
                    ))}
                  </Stack>
                )}
              </Box>
            </Stack>
          )}
        </DialogContent>
        <DialogActions>
          {(user?.role === 'student' || user?.role === 'teacher' || user?.role === 'admin') && teams.length > 0 && (
            <Button onClick={() => setMemberDialogOpen(true)}>Add member</Button>
          )}
          <Button onClick={() => setSelectedProject(null)}>Close</Button>
        </DialogActions>
      </Dialog>

      <Dialog open={createDialogOpen} onClose={() => setCreateDialogOpen(false)} fullWidth maxWidth="sm">
        <DialogTitle>Create project</DialogTitle>
        <DialogContent>
          <Stack spacing={2} sx={{ mt: 1 }}>
            <TextField label="Title" value={form.title} onChange={(event) => setForm((prev) => ({ ...prev, title: event.target.value }))} />
            <TextField label="Description" multiline minRows={4} value={form.description} onChange={(event) => setForm((prev) => ({ ...prev, description: event.target.value }))} />
            <TextField label="Subject ID" value={form.subject_id} onChange={(event) => setForm((prev) => ({ ...prev, subject_id: event.target.value }))} helperText="Use a subject ID from the subject list." />
            <TextField label="Max students per team" type="number" value={form.max_students_per_team} onChange={(event) => setForm((prev) => ({ ...prev, max_students_per_team: event.target.value }))} />
            <TextField label="Start date" type="date" InputLabelProps={{ shrink: true }} value={form.start_date} onChange={(event) => setForm((prev) => ({ ...prev, start_date: event.target.value }))} />
            <TextField label="End date" type="date" InputLabelProps={{ shrink: true }} value={form.end_date} onChange={(event) => setForm((prev) => ({ ...prev, end_date: event.target.value }))} />
          </Stack>
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setCreateDialogOpen(false)}>Cancel</Button>
          <Button onClick={handleCreateProject} variant="contained">Create</Button>
        </DialogActions>
      </Dialog>

      <Dialog open={memberDialogOpen} onClose={() => setMemberDialogOpen(false)} fullWidth maxWidth="xs">
        <DialogTitle>Add team member</DialogTitle>
        <DialogContent>
          <TextField
            sx={{ mt: 1 }}
            fullWidth
            label="Student ID"
            value={memberStudentId}
            onChange={(event) => setMemberStudentId(event.target.value)}
            helperText="Enter the target student ID. In mock mode, try user-student-1."
          />
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setMemberDialogOpen(false)}>Cancel</Button>
          <Button onClick={handleAddMember} variant="contained">Add</Button>
        </DialogActions>
      </Dialog>
    </>
  )
}

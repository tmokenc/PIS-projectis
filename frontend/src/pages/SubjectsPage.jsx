import {
  Alert,
  Box,
  Button,
  Card,
  CardContent,
  Dialog,
  DialogActions,
  DialogContent,
  DialogTitle,
  Stack,
  TextField,
  Typography,
} from '@mui/material'
import AddRoundedIcon from '@mui/icons-material/AddRounded'
import EditRoundedIcon from '@mui/icons-material/EditRounded'
import DeleteRoundedIcon from '@mui/icons-material/DeleteRounded'
import SchoolRoundedIcon from '@mui/icons-material/SchoolRounded'
import { useEffect, useMemo, useState } from 'react'
import PageHeader from '../components/PageHeader'
import LoadingState from '../components/LoadingState'
import EmptyState from '../components/EmptyState'
import { useAuth } from '../contexts/AuthContext'
import { api } from '../lib/api'

const initialForm = { name: '', description: '', abbreviation: '' }

export default function SubjectsPage() {
  const { token, user } = useAuth()
  const session = useMemo(() => ({ token, user }), [token, user])
  const [subjects, setSubjects] = useState([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState('')
  const [dialogOpen, setDialogOpen] = useState(false)
  const [editing, setEditing] = useState(null)
  const [form, setForm] = useState(initialForm)

  async function loadSubjects() {
    setLoading(true)
    setError('')
    try {
      const data = await api.listSubjects(session)
      setSubjects(Array.isArray(data) ? data : data.subjects || [])
    } catch (err) {
      setError(err.message || 'Failed to load subjects')
    } finally {
      setLoading(false)
    }
  }

  useEffect(() => {
    loadSubjects()
  }, [token, user])

  async function handleRegister(subjectId) {
    try {
      await api.registerSubject(session, subjectId)
      setError('')
    } catch (err) {
      setError(err.message || 'Failed to register to subject')
    }
  }

  async function handleSave() {
    try {
      if (editing) {
        await api.updateSubject(session, editing.id, form)
      } else {
        await api.createSubject(session, form)
      }
      setDialogOpen(false)
      setEditing(null)
      setForm(initialForm)
      await loadSubjects()
    } catch (err) {
      setError(err.message || 'Failed to save subject')
    }
  }

  async function handleDelete(subjectId) {
    try {
      await api.deleteSubject(session, subjectId)
      await loadSubjects()
    } catch (err) {
      setError(err.message || 'Failed to delete subject')
    }
  }

  if (loading) return <LoadingState />

  return (
    <>
      <PageHeader
        eyebrow="Subjects"
        title="Subject catalogue"
        subtitle="Browse all available subjects and register quickly. Admins can maintain the subject list from the same screen."
        actions={
          user?.role === 'admin' ? (
            <Button
              variant="contained"
              startIcon={<AddRoundedIcon />}
              onClick={() => {
                setEditing(null)
                setForm(initialForm)
                setDialogOpen(true)
              }}
            >
              Create subject
            </Button>
          ) : null
        }
      />

      {error && <Alert severity="error" sx={{ mb: 3 }}>{error}</Alert>}

      {!subjects.length ? (
        <EmptyState
          title="No subjects found"
          description="Create the first subject or check whether the backend service is reachable."
          actionLabel={user?.role === 'admin' ? 'Create subject' : undefined}
          onAction={
            user?.role === 'admin'
              ? () => {
                  setEditing(null)
                  setForm(initialForm)
                  setDialogOpen(true)
                }
              : undefined
          }
        />
      ) : (
        <Box
          sx={{
            display: 'grid',
            gridTemplateColumns: { xs: '1fr', md: 'repeat(2, 1fr)', xl: 'repeat(3, 1fr)' },
            gap: 2.5,
          }}
        >
          {subjects.map((subject) => (
            <Card key={subject.id} sx={{ height: '100%' }}>
              <CardContent sx={{ height: '100%', display: 'flex', flexDirection: 'column' }}>
                <Stack direction="row" justifyContent="space-between" spacing={2}>
                  <Stack direction="row" spacing={1.5} alignItems="center">
                    <Box
                      sx={{
                        width: 48,
                        height: 48,
                        borderRadius: 3,
                        display: 'grid',
                        placeItems: 'center',
                        bgcolor: 'rgba(21, 101, 192, 0.08)',
                        color: 'primary.main',
                      }}
                    >
                      <SchoolRoundedIcon />
                    </Box>
                    <Box>
                      <Typography variant="h6">{subject.name}</Typography>
                      <Typography variant="body2" color="text.secondary">
                        {subject.abbreviation}
                      </Typography>
                    </Box>
                  </Stack>
                </Stack>
                <Typography variant="body2" color="text.secondary" sx={{ mt: 2, flexGrow: 1 }}>
                  {subject.description}
                </Typography>
                <Stack direction="row" spacing={1} sx={{ mt: 3 }} flexWrap="wrap" useFlexGap>
                  {user?.role === 'student' && (
                    <Button variant="contained" onClick={() => handleRegister(subject.id)}>
                      Register
                    </Button>
                  )}
                  {user?.role === 'admin' && (
                    <>
                      <Button
                        variant="outlined"
                        startIcon={<EditRoundedIcon />}
                        onClick={() => {
                          setEditing(subject)
                          setForm({
                            name: subject.name,
                            description: subject.description,
                            abbreviation: subject.abbreviation,
                          })
                          setDialogOpen(true)
                        }}
                      >
                        Edit
                      </Button>
                      <Button color="error" variant="outlined" startIcon={<DeleteRoundedIcon />} onClick={() => handleDelete(subject.id)}>
                        Delete
                      </Button>
                    </>
                  )}
                </Stack>
              </CardContent>
            </Card>
          ))}
        </Box>
      )}

      <Dialog open={dialogOpen} onClose={() => setDialogOpen(false)} fullWidth maxWidth="sm">
        <DialogTitle>{editing ? 'Edit subject' : 'Create subject'}</DialogTitle>
        <DialogContent>
          <Stack spacing={2} sx={{ mt: 1 }}>
            <TextField label="Name" value={form.name} onChange={(event) => setForm((prev) => ({ ...prev, name: event.target.value }))} fullWidth />
            <TextField label="Abbreviation" value={form.abbreviation} onChange={(event) => setForm((prev) => ({ ...prev, abbreviation: event.target.value }))} fullWidth />
            <TextField label="Description" value={form.description} onChange={(event) => setForm((prev) => ({ ...prev, description: event.target.value }))} fullWidth multiline minRows={4} />
          </Stack>
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setDialogOpen(false)}>Cancel</Button>
          <Button onClick={handleSave} variant="contained">Save</Button>
        </DialogActions>
      </Dialog>
    </>
  )
}

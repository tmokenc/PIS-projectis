import {
  AppBar,
  Avatar,
  Badge,
  Box,
  Chip,
  Divider,
  Drawer,
  IconButton,
  List,
  ListItemButton,
  ListItemIcon,
  ListItemText,
  Menu,
  MenuItem,
  Stack,
  Toolbar,
  Tooltip,
  Typography,
  useMediaQuery,
} from '@mui/material'
import DashboardRoundedIcon from '@mui/icons-material/DashboardRounded'
import MenuRoundedIcon from '@mui/icons-material/MenuRounded'
import LogoutRoundedIcon from '@mui/icons-material/LogoutRounded'
import FolderOpenRoundedIcon from '@mui/icons-material/FolderOpenRounded'
import ClassRoundedIcon from '@mui/icons-material/ClassRounded'
import NotificationsRoundedIcon from '@mui/icons-material/NotificationsRounded'
import PeopleRoundedIcon from '@mui/icons-material/PeopleRounded'
import AssignmentTurnedInRoundedIcon from '@mui/icons-material/AssignmentTurnedInRounded'
import { useTheme } from '@mui/material/styles'
import { Outlet, useLocation, useNavigate } from 'react-router'
import { useEffect, useMemo, useState } from 'react'
import { useAuth } from '../contexts/AuthContext'
import { api } from '../lib/api'

const drawerWidth = 280

export default function AppShell() {
  const theme = useTheme()
  const mobile = useMediaQuery(theme.breakpoints.down('md'))
  const location = useLocation()
  const navigate = useNavigate()
  const { logout, user, token } = useAuth()
  const [drawerOpen, setDrawerOpen] = useState(false)
  const [menuAnchor, setMenuAnchor] = useState(null)
  const [unreadCount, setUnreadCount] = useState(0)

  useEffect(() => {
    let active = true

    async function loadNotifications() {
      if (!user) return
      try {
        const items = await api.listNotifications({ token, user }, user.id)
        if (active) {
          setUnreadCount(items.filter((item) => !item.read).length)
        }
      } catch {
        if (active) setUnreadCount(0)
      }
    }

    loadNotifications()
    return () => {
      active = false
    }
  }, [token, user, location.pathname])

  const navigation = useMemo(() => {
    const items = [
      { label: 'Dashboard', icon: <DashboardRoundedIcon />, path: '/dashboard' },
      { label: 'Subjects', icon: <ClassRoundedIcon />, path: '/subjects' },
      { label: 'Projects', icon: <FolderOpenRoundedIcon />, path: '/projects' },
      {
        label: 'Notifications',
        icon: <NotificationsRoundedIcon />,
        path: '/notifications',
        badge: unreadCount,
      },
    ]

    if (user?.role === 'admin') {
      items.push({
        label: 'User Management',
        icon: <PeopleRoundedIcon />,
        path: '/admin/users',
      })
    }

    return items
  }, [user?.role, unreadCount])

  const drawerContent = (
    <Box sx={{ height: '100%', display: 'flex', flexDirection: 'column' }}>
      <Box sx={{ p: 3, pb: 2 }}>
        <Stack direction="row" spacing={1.5} alignItems="center">
          <Avatar sx={{ bgcolor: 'secondary.main', width: 48, height: 48 }}>
            <AssignmentTurnedInRoundedIcon />
          </Avatar>
          <Box>
            <Typography variant="h6">Project Registration</Typography>
            <Typography variant="body2" color="text.secondary">
              University Information System
            </Typography>
          </Box>
        </Stack>
      </Box>
      <Divider />
      <List sx={{ px: 1.5, py: 2, flexGrow: 1 }}>
        {navigation.map((item) => {
          const active = location.pathname === item.path
          return (
            <ListItemButton
              key={item.path}
              onClick={() => {
                navigate(item.path)
                setDrawerOpen(false)
              }}
              sx={{
                borderRadius: 3,
                mb: 0.5,
                bgcolor: active ? 'primary.main' : 'transparent',
                color: active ? 'primary.contrastText' : 'text.primary',
                '&:hover': {
                  bgcolor: active ? 'primary.dark' : 'action.hover',
                },
              }}
            >
              <ListItemIcon sx={{ color: 'inherit', minWidth: 42 }}>
                {item.badge ? (
                  <Badge color="error" badgeContent={item.badge}>
                    {item.icon}
                  </Badge>
                ) : (
                  item.icon
                )}
              </ListItemIcon>
              <ListItemText primary={item.label} />
            </ListItemButton>
          )
        })}
      </List>
      <Divider />
      <Box sx={{ p: 2.5 }}>
        <Typography variant="body2" color="text.secondary">
          Signed in as
        </Typography>
        <Typography variant="subtitle1" sx={{ fontWeight: 700 }}>
          {user?.firstname} {user?.lastname}
        </Typography>
        <Stack direction="row" spacing={1} alignItems="center" sx={{ mt: 1 }}>
          <Chip size="small" color="secondary" label={user?.role || 'unknown'} />
          <Typography variant="caption" color="text.secondary">
            {user?.email}
          </Typography>
        </Stack>
      </Box>
    </Box>
  )

  return (
    <Box sx={{ display: 'flex', minHeight: '100vh', bgcolor: 'background.default' }}>
      <AppBar position="fixed" sx={{ width: { md: `calc(100% - ${drawerWidth}px)` }, ml: { md: `${drawerWidth}px` } }}>
        <Toolbar>
          {mobile && (
            <IconButton color="inherit" edge="start" onClick={() => setDrawerOpen(true)} sx={{ mr: 1 }}>
              <MenuRoundedIcon />
            </IconButton>
          )}
          <Box sx={{ flexGrow: 1 }}>
            <Typography variant="h6">{navigation.find((item) => item.path === location.pathname)?.label || 'Workspace'}</Typography>
            <Typography variant="body2" sx={{ opacity: 0.92 }}>
              Manage projects, registrations, and notifications in one place.
            </Typography>
          </Box>
          <Tooltip title="Account menu">
            <IconButton color="inherit" onClick={(event) => setMenuAnchor(event.currentTarget)}>
              <Avatar sx={{ bgcolor: 'rgba(255,255,255,0.2)' }}>
                {user?.firstname?.[0] || 'U'}
              </Avatar>
            </IconButton>
          </Tooltip>
          <Menu
            anchorEl={menuAnchor}
            open={Boolean(menuAnchor)}
            onClose={() => setMenuAnchor(null)}
            anchorOrigin={{ vertical: 'bottom', horizontal: 'right' }}
            transformOrigin={{ vertical: 'top', horizontal: 'right' }}
          >
            <MenuItem disabled>{user?.email}</MenuItem>
            <MenuItem
              onClick={async () => {
                setMenuAnchor(null)
                await logout()
                navigate('/login')
              }}
            >
              <ListItemIcon>
                <LogoutRoundedIcon fontSize="small" />
              </ListItemIcon>
              Sign out
            </MenuItem>
          </Menu>
        </Toolbar>
      </AppBar>

      <Box component="nav" sx={{ width: { md: drawerWidth }, flexShrink: { md: 0 } }}>
        <Drawer
          variant={mobile ? 'temporary' : 'permanent'}
          open={mobile ? drawerOpen : true}
          onClose={() => setDrawerOpen(false)}
          ModalProps={{ keepMounted: true }}
          sx={{
            '& .MuiDrawer-paper': {
              width: drawerWidth,
              boxSizing: 'border-box',
              borderRight: '1px solid rgba(15, 23, 42, 0.08)',
            },
          }}
        >
          {drawerContent}
        </Drawer>
      </Box>

      <Box component="main" sx={{ flexGrow: 1, p: { xs: 2, sm: 3 }, mt: 10, minWidth: 0 }}>
        <Outlet />
      </Box>
    </Box>
  )
}

import { Box, CircularProgress } from '@mui/material'
import { Navigate, useLocation } from 'react-router'
import { useAuth } from '../contexts/AuthContext'

export default function ProtectedRoute({ children, roles }) {
  const { isAuthenticated, initializing, user } = useAuth()
  const location = useLocation()

  if (initializing) {
    return (
      <Box
        sx={{
          minHeight: '100vh',
          display: 'grid',
          placeItems: 'center',
          bgcolor: 'background.default',
        }}
      >
        <CircularProgress />
      </Box>
    )
  }

  if (!isAuthenticated) {
    return <Navigate to="/login" replace state={{ from: location }} />
  }

  if (roles?.length && !roles.includes(user?.role)) {
    return <Navigate to="/dashboard" replace />
  }

  return children
}

import { Box, Button, Stack, Typography } from '@mui/material'
import { useNavigate } from 'react-router'

export default function NotFoundPage() {
  const navigate = useNavigate()

  return (
    <Box sx={{ minHeight: '100vh', display: 'grid', placeItems: 'center', bgcolor: 'background.default' }}>
      <Stack spacing={2} alignItems="center">
        <Typography variant="h2">404</Typography>
        <Typography variant="h6">Page not found</Typography>
        <Typography variant="body2" color="text.secondary">
          The requested page does not exist.
        </Typography>
        <Button variant="contained" onClick={() => navigate('/dashboard')}>
          Go to dashboard
        </Button>
      </Stack>
    </Box>
  )
}

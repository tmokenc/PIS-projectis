import { Button, Paper, Stack, Typography } from '@mui/material'

export default function EmptyState({ title, description, actionLabel, onAction }) {
  return (
    <Paper sx={{ p: 5, textAlign: 'center' }}>
      <Stack spacing={1.5} alignItems="center">
        <Typography variant="h6">{title}</Typography>
        <Typography variant="body2" color="text.secondary" sx={{ maxWidth: 520 }}>
          {description}
        </Typography>
        {actionLabel && onAction && (
          <Button variant="contained" onClick={onAction} sx={{ mt: 1 }}>
            {actionLabel}
          </Button>
        )}
      </Stack>
    </Paper>
  )
}

import { Card, CardContent, Stack, Typography } from '@mui/material'

export default function StatCard({ icon, label, value, color = 'primary.main' }) {
  return (
    <Card>
      <CardContent>
        <Stack direction="row" justifyContent="space-between" alignItems="center">
          <Stack spacing={0.75}>
            <Typography variant="body2" color="text.secondary">
              {label}
            </Typography>
            <Typography variant="h4">{value}</Typography>
          </Stack>
          <Stack
            sx={{
              width: 56,
              height: 56,
              borderRadius: 4,
              bgcolor: `${color}15`,
              color,
            }}
            alignItems="center"
            justifyContent="center"
          >
            {icon}
          </Stack>
        </Stack>
      </CardContent>
    </Card>
  )
}

import { Box, Stack, Typography } from '@mui/material'

export default function PageHeader({ eyebrow, title, subtitle, actions }) {
  return (
    <Stack
      direction={{ xs: 'column', md: 'row' }}
      justifyContent="space-between"
      alignItems={{ xs: 'flex-start', md: 'center' }}
      spacing={2}
      sx={{ mb: 3 }}
    >
      <Box>
        {eyebrow && (
          <Typography variant="overline" color="primary.main" sx={{ letterSpacing: 1.5 }}>
            {eyebrow}
          </Typography>
        )}
        <Typography variant="h4" sx={{ mt: 0.5 }}>
          {title}
        </Typography>
        {subtitle && (
          <Typography variant="body1" color="text.secondary" sx={{ mt: 1 }}>
            {subtitle}
          </Typography>
        )}
      </Box>
      {actions}
    </Stack>
  )
}

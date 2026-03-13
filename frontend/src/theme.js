import { alpha, createTheme } from '@mui/material/styles'

export const theme = createTheme({
  palette: {
    mode: 'light',
    primary: {
      main: '#1565c0',
      light: '#5e92f3',
      dark: '#003c8f',
    },
    secondary: {
      main: '#00897b',
      light: '#4ebaaa',
      dark: '#005b4f',
    },
    background: {
      default: '#f4f7fb',
      paper: '#ffffff',
    },
    success: {
      main: '#2e7d32',
    },
    warning: {
      main: '#ed6c02',
    },
    error: {
      main: '#d32f2f',
    },
  },
  shape: {
    borderRadius: 16,
  },
  typography: {
    fontFamily: [
      'Inter',
      'Roboto',
      'Helvetica',
      'Arial',
      'sans-serif',
    ].join(','),
    h4: {
      fontWeight: 700,
    },
    h5: {
      fontWeight: 700,
    },
    h6: {
      fontWeight: 700,
    },
    button: {
      textTransform: 'none',
      fontWeight: 600,
    },
  },
  components: {
    MuiAppBar: {
      styleOverrides: {
        root: {
          backgroundImage:
            'linear-gradient(135deg, rgba(21,101,192,1) 0%, rgba(0,137,123,1) 100%)',
          boxShadow: '0 12px 24px rgba(21, 101, 192, 0.15)',
        },
      },
    },
    MuiPaper: {
      styleOverrides: {
        root: {
          backgroundImage: 'none',
        },
      },
    },
    MuiCard: {
      styleOverrides: {
        root: ({ theme }) => ({
          border: `1px solid ${alpha(theme.palette.primary.main, 0.08)}`,
          boxShadow: '0 10px 30px rgba(15, 23, 42, 0.06)',
        }),
      },
    },
    MuiButton: {
      defaultProps: {
        disableElevation: true,
      },
    },
  },
})

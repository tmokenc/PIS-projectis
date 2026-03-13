# Frontend

React + Vite + Material UI frontend for the university project registration system.

## Features

- Login and register
- Role-based dashboards for student, teacher, and admin
- Subject browsing and subject registration
- Project browsing and team registration
- Notifications inbox with unread counters
- Admin pages for subject management and user management UX
- Teacher project overview and quick actions
- Material UI layout optimized for desktop and mobile

## Environment

Copy `.env.example` to `.env`.

- `VITE_API_BASE_URL`: router base URL
- `VITE_API_MODE`:
  - `live`: only call the backend
  - `mock`: only use local mock data
  - `hybrid`: try backend first, then fallback to mock data if endpoint is missing or unavailable

## Run

```bash
npm install
npm run dev
```

## Notes

The current backend in this repository does not expose every use case yet, especially user management and project CRUD. The frontend still includes those screens and uses mock fallbacks when `VITE_API_MODE=hybrid` or `mock`.

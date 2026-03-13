export function formatDate(value) {
  if (!value) return '—'
  const date = new Date(value)
  return new Intl.DateTimeFormat('en-GB', {
    dateStyle: 'medium',
    timeStyle: 'short',
  }).format(date)
}

export function formatDateOnly(value) {
  if (!value) return '—'
  const date = new Date(value)
  return new Intl.DateTimeFormat('en-GB', {
    dateStyle: 'medium',
  }).format(date)
}

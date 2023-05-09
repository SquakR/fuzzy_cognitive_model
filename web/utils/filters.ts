import { UserOutType } from '~/types'

export const dateTimeFilter = (dt: string): string => {
  const formatter = new Intl.DateTimeFormat('ru', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit',
  })
  return formatter.format(new Date(dt)).replace(',', '')
}

export const userNameFilter = (user: UserOutType) => {
  return `${user.firstName} ${user.lastName}`
}

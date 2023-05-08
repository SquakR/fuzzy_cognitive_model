export interface UserOutType {
  id: number
  username: string
  email: string
  isEmailConfirmed: boolean
  firstName: string
  secondName: string | null
  lastName: string
  avatar: string | null
  language: string | null
  createdAt: string
  updatedAt: string
}

export interface UserInCreateType {
  username: string
  password: string
  email: string
  firstName: string
  secondName: string | null
  lastName: string
  avatar: File | null
  language: string | null
}

export interface UseInChangeType {
  username: string
  email: string
  firstName: string
  secondName: string | null
  lastName: string
  avatar: File | null
  resetAvatar: boolean
}

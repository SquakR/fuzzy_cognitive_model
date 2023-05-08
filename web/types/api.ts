export interface UserOutType {
  id: number
  username: string
  email: string
  isEmailConfirmed: boolean
  firstName: string
  secondName: string | null
  lastName: string
  avatar: string | null
  locale: string | null
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
  locale: string | null
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

export interface DeviceType {
  name: string | null
  brand: string | null
  model: string | null
}

export interface OSType {
  name: string | null
  major: string | null
  minor: string | null
  patch: string | null
  patch_minor: string | null
}

export interface ProductType {
  name: string | null
  major: string | null
  minor: string | null
  patch: string | null
}

export interface SessionType {
  id: number
  isCurrent: boolean
  createdAt: string
  ipAddress: string
  device: DeviceType
  os: OSType
  product: ProductType
}

export interface CredentialsType {
  username: string
  password: string
}

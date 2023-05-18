import { DynamicModelType } from './plugins/adjustment'

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

export interface ChangePasswordType {
  oldPassword: string
  newPassword: string
}

export interface ResetPasswordType {
  token: string
  newPassword: string
}

export type ConceptValueType = 'none' | 'from_zero_to_one'

export type ConnectionValueType = 'symbolic' | 'from_minus_one_to_one'

export interface ProjectOutType {
  id: number
  name: string
  description: string
  creator: UserOutType
  isPublic: boolean
  isArchived: boolean
  createdAt: string
  updatedAt: string
  conceptValueType: ConceptValueType
  connectionValueType: ConnectionValueType
  plugins: string[]
}

export interface ProjectInType {
  name: string
  description: string
  isPublic: boolean
  isArchived: boolean
  conceptValueType: ConceptValueType
  connectionValueType: ConnectionValueType
}

export type ProjectGroupFilterType = 'public' | 'private' | 'both'

export interface PluginType {
  name: string
  description: string
  conceptValueType: ConceptValueType | null
  connectionValueType: ConnectionValueType | null
  dependencies: string[]
}

export interface PermissionType {
  key: string
  description: string
}

export type ProjectUserStatusValue =
  | 'creator'
  | 'invited'
  | 'cancelled'
  | 'rejected'
  | 'member'
  | 'excluded'
  | 'left'

export interface ProjectUserType {
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
  status: ProjectUserStatusValue
  permissions: string[] | null
}

export interface ProjectsInType {
  group: ProjectGroupFilterType
  statuses: ProjectUserStatusValue[] | null
  search: string | null
  isArchived: boolean | null
  createdAtStart: string | null
  createdAtIncludeStart: boolean | null
  createdAtEnd: string | null
  createdAtIncludeEnd: boolean | null
  updatedAtStart: string | null
  updatedAtIncludeStart: boolean | null
  updatedAtEnd: string | null
  updatedAtIncludeEnd: boolean | null
  page: number | null
  perPage: number | null
}

export interface ConceptOutType {
  id: number
  name: string
  description: string
  value: number | null
  projectId: number
  xPosition: number
  yPosition: number
  pluginsData: {
    controlConcepts?: {
      isControl: boolean
    }
    targetConcepts?: {
      isTarget: boolean
      value: number
    }
    conceptConstraints?: {
      hasConstraint: boolean
      minValue: number
      includeMinValue: boolean
      maxValue: number
      includeMaxValue: boolean
    }
    adjustment?: {
      dynamicModelType: DynamicModelType
    }
  }
  createdAt: string
  updatedAt: string
}

export interface ConceptInType {
  name: string
  description: string
  value: number | null
  xPosition: number
  yPosition: number
}

export interface ConceptOutMoveType {
  id: number
  xPosition: number
  yPosition: number
  updatedAt: string
}

export interface ConceptInMoveType {
  xPosition: number
  yPosition: number
}

export interface ConceptOutDeleteType {
  id: number
  updatedAt: string
}

export interface ConnectionOutType {
  id: number
  description: string
  value: number
  sourceId: number
  targetId: number
  projectId: number
  pluginsData: {
    controlConnections?: {
      isControl: boolean
    }
    connectionConstraints: {
      hasConstraint: boolean
      minValue: number
      includeMinValue: boolean
      maxValue: number
      includeMaxValue: boolean
    }
  }
  createdAt: string
  updatedAt: string
}

export interface ConnectionInCreateType {
  description: string
  value: number
  sourceId: number
  targetId: number
}

export interface ConnectionOutChangeDescriptionType {
  id: number
  description: string
  updatedAt: string
}

export interface ConnectionOutChangeValueType {
  id: number
  value: number
  updateAt: string
}

export interface ConnectionOutDeleteType {
  id: number
  updatedAt: string
}

export interface ModelOutType {
  project: ProjectOutType
  concepts: ConceptOutType[]
  connections: ConnectionOutType[]
}

export interface PaginationOutType<T extends { id: number }> {
  data: T[]
  totalCount: number
  totalPages: number
}

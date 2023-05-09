export interface LocalFetchOptions {
  key: string
  emitError?: boolean
}

export interface LocalFetchFuncOptions {
  key: string
  successMessage?: string
  emitError?: boolean
}

export interface LocalFetchResultSuccess<T> {
  data: T
  success: true
  errorData: null
}

export interface LocalFetchResultError {
  data: null
  success: false
  errorData: string | ErrorPayload
}

export type LocalFetchResult<T> =
  | LocalFetchResultSuccess<T>
  | LocalFetchResultError

export interface ErrorPayload {
  error: { code: number; reason: string; description: string }
}

export type ErrorData = ErrorPayload | string

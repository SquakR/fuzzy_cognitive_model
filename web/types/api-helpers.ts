export type LocalFetchFuncOptions<T> = {
  key: string
  successMessage?: string
  emitError?: boolean
  onSuccess?: (data: T) => void
  onError?: (error: string) => void
}

export interface SuccessMessage {
  key: string
  type: 'success'
  message: string
}

export interface ErrorMessage {
  key: string
  type: 'error'
  message: string
}

export interface ClearMessage {
  key: string
  type: 'clear'
}

export type Message = SuccessMessage | ErrorMessage | ClearMessage

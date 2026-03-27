import { useToast } from 'primevue/usetoast'

interface MessageOptions {
  duration?: number
}

interface MessageApiInjection {
  success: (content: string, options?: MessageOptions) => void
  error: (content: string, options?: MessageOptions) => void
  info: (content: string, options?: MessageOptions) => void
  warning: (content: string, options?: MessageOptions) => void
  warn: (content: string, options?: MessageOptions) => void
}

const DEFAULT_LIFE = 3000

function createMessage(summary: string, severity: 'success' | 'error' | 'info' | 'warn', content: string, options?: MessageOptions) {
  return {
    severity,
    summary,
    detail: content,
    life: options?.duration ?? DEFAULT_LIFE,
  }
}

function useMessage(): MessageApiInjection {
  const toast = useToast()

  return {
    success(content, options) {
      toast.add(createMessage('Success', 'success', content, options))
    },
    error(content, options) {
      toast.add(createMessage('Error', 'error', content, options))
    },
    info(content, options) {
      toast.add(createMessage('Info', 'info', content, options))
    },
    warning(content, options) {
      toast.add(createMessage('Warning', 'warn', content, options))
    },
    warn(content, options) {
      toast.add(createMessage('Warning', 'warn', content, options))
    },
  }
}

export { useMessage }
export type { MessageApiInjection, MessageOptions }

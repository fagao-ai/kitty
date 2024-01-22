import { isReactive, reactive } from 'vue'

export type FormItemType = 'input' | 'select' | 'textarea' | 'checkbox' | 'radio'

export interface FormItem<T = any> {
  type: FormItemType
  payload: T
  next: (current: FormItem<T>, acients: FormItem<T>[]) => FormItem<T> | null
  parent: FormItem<T> | null
}

export function useFormItem<T>() {
  function createFormItem(formItemType: FormItem<T>['type'], payload: FormItem<T>['payload'], next?: FormItem<T>['next'], parent?: FormItem<T>['parent']): FormItem<T> {
    if (!next)
      next = () => null

    if (!parent)
      parent = null

    function nextFuncion(current: FormItem<T>, acients: FormItem<T>[]) {
      let nextItem = next!(current, acients)
      if (!nextItem)
        return null

      nextItem.parent = current
      if (!isReactive(nextItem))
        nextItem = reactive(nextItem) as FormItem<T>

      return nextItem
    }

    const formItem: FormItem<T> = reactive({
      type: formItemType,
      payload,
      next: nextFuncion,
      parent,
    }) as FormItem<T>

    return formItem
  }

  return { createFormItem }
}

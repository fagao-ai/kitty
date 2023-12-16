export interface KittyResponse<T> {
  code: number
  data: T
  msg: string
}

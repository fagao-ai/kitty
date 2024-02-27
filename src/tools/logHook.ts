export class KittyLogQueue {
  private queue: string[]
  private maxSize: number

  constructor(size: number) {
    this.queue = []
    this.maxSize = size
  }

  push(item: string): void {
    if (this.queue.length >= this.maxSize)
      this.queue.shift()

    this.queue.push(item)
  }

  getQueue(): string[] {
    return this.queue
  }
}

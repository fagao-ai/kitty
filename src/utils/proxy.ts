import { ProxyType } from '@/types/proxy'

/**
 * Get the short name for a protocol tag.
 * Returns abbreviated protocol names for display in the UI.
 *
 * @param tag - The protocol tag (e.g., "hysteria", "vless", "vmess", "trojan")
 * @param type - The proxy type (Hysteria or Xray)
 * @returns The short protocol name (e.g., "HY", "VL", "VM", "TR")
 */
export function getProtocolShortName(tag: string, type: ProxyType): string {
  if (type === ProxyType.Hysteria) {
    return 'HY'
  }

  const shortNames: Record<string, string> = {
    vless: 'VL',
    vmess: 'VM',
    trojan: 'TR',
  }

  return shortNames[tag.toLowerCase()] || tag.substring(0, 2).toUpperCase()
}

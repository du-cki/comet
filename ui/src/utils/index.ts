import { sortFiles, SortBy, setSortType } from './sort'

// Since the `WebSocket` constructor considers relative paths
// (i.e `/ws`) to be illegal (probably cause it expands to an http(s) URL),
// we need to replace them with a full URL.
const url = (relativePath: string): string => {
  const loc = window.location
  return (loc.protocol === 'https:' ? 'wss://' : 'ws://') + loc.host + relativePath
}

const sleep = (seconds: number) => new Promise((resolve) => setTimeout(resolve, seconds * 1000))

const updateState = ({ hash, title }: { hash?: string; title?: string }) => {
  // the typeof is required, but '' == false which would prevent me from resetting the hash
  if (typeof hash === 'string') window.location.hash = hash
  if (title) document.title = ''
}

export { sortFiles, SortBy, setSortType, url, sleep, updateState }

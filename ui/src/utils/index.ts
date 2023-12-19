import {
    sortFiles,
    SortBy,
    setSortType,
} from "./sort";

// Since the `WebSocket` constructor considers relative paths
// (i.e `/ws`) to be illegal (probably cause it expands to an http(s) URL), 
// we need to replace them with a full URL.
const url = (relativePath: string): string => {
    const loc = window.location;
    return ((loc.protocol === "https:") ? "wss://" : "ws://") + loc.host + relativePath;
}

const sleep = (seconds: number) => new Promise(resolve => setTimeout(resolve, seconds * 1000));

const updateUrlHash = (hash: string) => {
    window.history.pushState({ path: hash }, '', hash)
    window.location.hash = hash
}

export {
    sortFiles,
    SortBy,
    setSortType,
    url,
    sleep,
    updateUrlHash,
}
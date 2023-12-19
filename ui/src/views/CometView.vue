<script setup lang="ts">
import { ref, inject } from 'vue'

import { updateUrlHash } from '@/utils'

import FileTable from '@/components/FileTable.vue'

import type Client from '@/lib/comet'
import type { FileT, FolderT } from '@/lib/comet/types'

let path = window.location.hash.substring(1)
if (!path) {
  path = '/'
} else if (path === '/') {
  // clear it if it's the root, to keep it clean.
  updateUrlHash('')
}

document.title = `${path} - Comet`

const displayedFiles = ref<{
  files: FileT[]
  folders: FolderT[]
}>({ files: [], folders: [] })

const client = inject('client') as Client
client.requestFiles(path).then((res) => {
  displayedFiles.value = res
})
</script>

<template>
  <main class="pt-8">
    <FileTable :folders="displayedFiles.folders" :files="displayedFiles.files" />
  </main>
</template>

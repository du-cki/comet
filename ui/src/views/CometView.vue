<script setup lang="ts">
import { ref, inject } from 'vue'

import { updateState } from '@/utils'

import FileTable from '@/components/FileTable.vue'

import type Client from '@/lib/comet'
import type { FileT } from '@/lib/comet/types'

let path = window.location.hash.substring(1)
if (!path) {
  path = '/'
} else if (path === '/') {
  // clear it if it's the root, to keep it clean.
  updateState({
    hash: ''
  })
}

const files = ref<FileT[]>([])
const client = inject('client') as Client

const openFile = (file: string) => {
  path = file

  updateState({
    title: `${path} - Comet`,
    hash: path
  })

  client.requestFiles(path).then((res) => {
    files.value = res.files
  })
}

openFile(path)
</script>

<template>
  <main class="pt-8">
    <FileTable
      :files="files"
      :back="path !== '/'"
      @open-file="
        (file: FileT) => {
          if (file.file_type === 'FOLDER') {
            if (file.name === '...') {
              return openFile(path.substring(0, path.lastIndexOf('/')) || '/')
            }

            return openFile(`${path.endsWith('/') ? path : path + '/'}${file.name}`)
          }
        }
      "
    />
  </main>
</template>

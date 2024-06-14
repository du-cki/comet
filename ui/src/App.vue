<script setup lang="ts">
import { ref, inject } from 'vue'

import { updateState } from './utils'

import FileTable from './components/FileTable.vue'

import type Client from './lib/comet'
import { FileType, type FileT } from './lib/comet/types'

const path = ref<string>(window.location.hash.substring(1) || '/')


const files = ref<FileT[]>([])
const client = inject('client') as Client

client.onmessage = (message: object) => {
  console.log(message)
}

window.onhashchange = () => {
  const newPath = window.location.hash.substring(1) || '/'

  if (newPath !== path.value) {
    goToFolder(window.location.hash.substring(1) || '/')
  }
}

const goToFolder = async (folder: string) => {
  files.value = []

  // TODO: cache folders in session storage
  const req = await client.requestFiles(folder)
  files.value = req.files

  path.value = folder

  updateState({
    title: `${folder} - Comet`,
    hash: folder == '/' ? '' : folder
  })
}

goToFolder(path.value)
</script>

<template>
  <main class="py-8">
    <FileTable :files="files" :back="path !== '/'" @folder-opened="async (file: FileT) => {
      if (file.file_type === FileType.FOLDER) {
        if (file.name === '...') {
          return await goToFolder(path.substring(0, path.lastIndexOf('/')) || '/')
        }

        return await goToFolder(`${path.endsWith('/') ? path : path + '/'}${file.name}`)
      }
    }
      " />
  </main>
</template>

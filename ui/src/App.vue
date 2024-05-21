<script setup lang="ts">
import { ref, inject } from 'vue'

import { updateState } from '@/utils'

import FileTable from '@/components/FileTable.vue'

import type Client from '@/lib/comet'
import { FileType, type FileT } from '@/lib/comet/types'

const path = ref<string>(window.location.hash.substring(1) || '/')

const files = ref<FileT[]>([])
const client = inject('client') as Client

const cachedDirectories = ref<{
  [key: string]: FileT[]
}>({})

client.onmessage = (message: object) => {
  console.log(message)
}

window.onhashchange = () => {
  const newPath = window.location.hash.substring(1) || '/'

  if (newPath !== path.value) {
    openFile(window.location.hash.substring(1) || '/')
  }
}

const openFile = async (folder: string) => {
  files.value = []

  // TODO: turn caching into a configurable value
  if (cachedDirectories.value[folder]) {
    files.value = cachedDirectories.value[folder]
  } else {
    const req = await client.requestFiles(folder)
    files.value = req.files

    cachedDirectories.value[folder] = files.value
  }

  path.value = folder

  updateState({
    title: `${folder} - Comet`,
    hash: folder == '/' ? '' : folder
  })
}

openFile(path.value)
</script>

<template>
  <main class="py-8">
    <FileTable
      :files="files"
      :back="path !== '/'"
      @open-file="
        async (file: FileT) => {
          if (file.file_type === FileType.FOLDER) {
            if (file.name === '...') {
              return await openFile(path.substring(0, path.lastIndexOf('/')) || '/')
            }

            return await openFile(`${path.endsWith('/') ? path : path + '/'}${file.name}`)
          }
        }
      "
    />
  </main>
</template>

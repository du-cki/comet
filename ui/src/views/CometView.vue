<script setup lang="ts">
import { ref, inject } from 'vue'

import { updateState } from '@/utils'

import FileTable from '@/components/FileTable.vue'

import type Client from '@/lib/comet'
import type { FileT } from '@/lib/comet/types'

let path = window.location.hash.substring(1) || '/'

const files = ref<FileT[]>([])
const client = inject('client') as Client

const openFile = async (file: string) => {
  updateState({
    title: `${file} - Comet`,
    hash: file == '/' ? '' : file
  })

  const req = await client.requestFiles(file)
  files.value = req.files

  path = file
}

openFile(path)
</script>

<template>
  <main class="py-8">
    <FileTable
      :files="files"
      :back="path !== '/'"
      @open-file="
        async (file: FileT) => {
          if (file.file_type === 'FOLDER') {
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

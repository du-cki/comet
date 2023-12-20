<script setup lang="ts">
import moment from 'moment'

import { Dropdown } from 'floating-vue'

import { FileIcon, FolderIcon, ThreeDots } from '@/icons'
import ContextMenu from '@/components/ContextMenu.vue'

import type { FileT } from '@/lib/comet/types'

defineEmits(['openFile'])

defineProps<{
  file: FileT
  time: boolean
}>()
</script>

<template>
  <td class="flex items-center py-2 pl-3">
    <FileIcon v-if="file.file_type === 'FILE'" class="fill-[#768390]" />
    <FolderIcon v-else-if="file.file_type === 'FOLDER'" class="fill-[#768390]" />

    <p
      class="pl-2 text-gray-700 truncate text-ellipsis hover:cursor-pointer hover:underline"
      v-on:click="$emit('openFile', file)"
    >
      {{ `${file.name}` }}
    </p>
  </td>

  <td class="pr-2 md:pr-3 pl-2 text-right">
    <span class="text-gray-500" v-if="time">
      {{ moment.unix(file.last_updated).fromNow() }}
    </span>
  </td>

  <td class="pr-2 pl-2 flex justify-center">
    <Dropdown v-if="file.file_type != 'FOLDER'" placement="left-start">
      <button>
        <ThreeDots
          class="h-4 fill-gray-500 hover:fill-gray-900 active:fill-gray-900 transition-all hover:cursor-pointer"
        />
      </button>

      <template #popper>
        <ContextMenu :is_logged_in="false" :file_id="file.id" />
      </template>
    </Dropdown>
  </td>
</template>

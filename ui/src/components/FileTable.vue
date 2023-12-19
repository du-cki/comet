<script setup lang="ts">
import { ref, type Ref } from 'vue'
import moment from 'moment'

import { Dropdown } from 'floating-vue'

import { FileIcon, FolderIcon, ThreeDots } from '@/icons'
import ContextMenu from '@/components/ContextMenu.vue'

import { sortFiles, SortBy, setSortType } from '@/utils'
import type { FileT, FolderT } from '@/lib/comet/types'

defineProps<{
  files: FileT[]
  folders: FolderT[]
}>()

const storedSortType = localStorage.getItem('sortType')
const sortType: Ref<SortBy | string> = ref(
  storedSortType ? (SortBy as any)[storedSortType].toString() : SortBy.UpdatedAsc.toString()
)
</script>

<template>
  <div class="flex w-full justify-center py-1">
    <div class="w-full mx-3 md:w-3/4 md:mx-0 border rounded-lg overflow-hidden">
      <table
        class="divider w-full rounded-lg overflow-hidden text-sm lg:text-lg select-none cursor-default"
      >
        <tr class="text-gray-500 text-sm font-semibold bg-gray-50">
          <th
            class="pl-3 py-1 text-left hover:cursor-pointer select-none"
            @click="
              () => {
                sortType = setSortType(
                  sortType === SortBy.NameAsc ? SortBy.NameDesc : SortBy.NameAsc
                )
              }
            "
          >
            Name
          </th>

          <th
            class="pr-2 md:pr-3 text-right hover:cursor-pointer select-none"
            @click="
              () => {
                sortType = setSortType(
                  sortType === SortBy.UpdatedAsc ? SortBy.UpdatedDesc : SortBy.UpdatedAsc
                )
              }
            "
          >
            Last Updated
          </th>

          <th class="w-1 bg-gray-100 opacity-50" />
        </tr>

        <tr class="hover:bg-gray-100" v-for="folder in folders" :key="folder.folder_id">
          <td class="flex items-center py-2 pl-3">
            <FolderIcon class="fill-[#768390]" />
            <p class="pl-2 text-gray-700 truncate text-ellipsis">
              {{ folder.folder_name }}
            </p>
          </td>

          <td class="pr-2 md:pr-3 pl-2 text-gray-500 text-right">
            {{
              folder.last_updated
                ? moment.unix(folder.last_updated).fromNow()
                : 'N/A' /* in-cases of there being no files inside the folder. */
            }}
          </td>

          <td class="pr-2 pl-2 flex justify-center">
            <Dropdown placement="left-start">
              <button>
                <ThreeDots
                  class="h-4 fill-gray-500 hover:fill-gray-900 active:fill-gray-900 transition-all hover:cursor-pointer"
                />
              </button>

              <template #popper>
                <ContextMenu :is_logged_in="true" :folder_id="folder.folder_id" />
              </template>
            </Dropdown>
          </td>
        </tr>

        <tr
          class="hover:bg-gray-100"
          v-for="file in sortFiles(files, sortType)"
          :key="file.file_id"
        >
          <td class="flex items-center py-2 pl-3">
            <FileIcon class="fill-[#768390]" />
            <p class="pl-2 text-gray-700 truncate text-ellipsis">
              {{ `${file.file_name}.${file.file_ext}` }}
            </p>
          </td>

          <td class="pr-2 md:pr-3 pl-2 text-gray-500 text-right">
            {{ moment.unix(file.last_updated).fromNow() }}
          </td>

          <td class="pr-2 pl-2 flex justify-center">
            <Dropdown placement="left-start">
              <button>
                <ThreeDots
                  class="h-4 fill-gray-500 hover:fill-gray-900 active:fill-gray-900 transition-all hover:cursor-pointer"
                />
              </button>

              <template #popper>
                <ContextMenu :is_logged_in="true" :file_id="file.file_id" />
              </template>
            </Dropdown>
          </td>
        </tr>
      </table>
    </div>
  </div>
</template>

<style scoped>
.divider {
  @apply divide-y divide-gray-200;
}
</style>
@/lib/comet/types

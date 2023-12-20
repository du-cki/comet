<script setup lang="ts">
import FileItem from './FileItem.vue'
import { ref, type Ref } from 'vue'

import { sortFiles, SortBy, setSortType } from '@/utils'
import type { FileT } from '@/lib/comet/types'

defineProps<{
  files: FileT[]
  back: boolean
}>()

defineEmits(['openFile'])

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
          <th class="pl-3 py-1 text-left select-none">
            <span
              class="hover:cursor-pointer"
              @click="
                () => {
                  sortType = setSortType(
                    sortType === SortBy.NameAsc ? SortBy.NameDesc : SortBy.NameAsc
                  )
                }
              "
            >
              Name
            </span>
          </th>

          <th class="pr-2 md:pr-3 text-right select-none">
            <span
              class="hover:cursor-pointer"
              @click="
                () => {
                  sortType = setSortType(
                    sortType === SortBy.UpdatedAsc ? SortBy.UpdatedDesc : SortBy.UpdatedAsc
                  )
                }
              "
            >
              Last Updated
            </span>
          </th>

          <th class="w-1 bg-gray-100 opacity-50" />
        </tr>

        <tr v-if="back" class="hover:bg-gray-100">
          <FileItem
            :file="{
              name: '...',
              id: 0,
              file_type: 'FOLDER',
              last_updated: 0
            }"
            :time="false"
            @open-file="$emit('openFile', $event)"
          />
        </tr>

        <tr v-for="file in sortFiles(files, sortType)" class="hover:bg-gray-100" :key="file.id">
          <FileItem :file="file" :time="true" @open-file="$emit('openFile', $event)" />
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

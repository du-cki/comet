<script setup lang="ts">
import { ShareIcon, DownloadIcon, RenameIcon, FolderOpenIcon, DeleteIcon, InfoIcon } from '@/icons'

defineProps<{
  is_logged_in: boolean
  file_id?: number
  folder_id?: number
}>()

enum ButtonType { Button, Divider }

const buttons = [
  {
    name: 'Share',
    icon: ShareIcon,
    type: ButtonType.Button,
    callback: () => {
      console.log('shared')
    }
  },
  {
    name: 'Download',
    icon: DownloadIcon,
    type: ButtonType.Button,
    callback: () => {
      console.log('downloaded')
    }
  },
  {
    isAuth: true,
    type: ButtonType.Divider
  },
  {
    name: 'Rename',
    icon: RenameIcon,
    isAuth: true,
    type: ButtonType.Button,
    callback: () => {
      console.log('renamed')
    }
  },
  {
    name: 'Move',
    icon: FolderOpenIcon,
    isAuth: true,
    type: ButtonType.Button,
    callback: () => {
      console.log('moved')
    }
  },
  {
    name: 'Delete',
    icon: DeleteIcon,
    isAuth: true,
    type: ButtonType.Button,
    customColour: 'fill-red-600 text-red-600',
    callback: () => {
      console.log('deleted')
    }
  },
  {
    type: ButtonType.Divider
  },
  {
    name: 'Info',
    icon: InfoIcon,
    type: ButtonType.Button,
    callback: () => {
      console.log('info')
    }
  }
]
</script>

<template>
  <div
    class="w-fit border-1 select-none rounded-md bg-white border-neutral-100 px-1 py-1 shadow text-sm"
  >
    <ul>
      <li v-for="button in buttons" :key="button.name">
        <div v-if="(button.isAuth && is_logged_in) || !button.isAuth">
          <div v-if="button.type === ButtonType.Button">
            <button
              :class="`item ${button.customColour || 'text-gray-700'}`"
              :on-click="button.callback"
            >
              <component
                :is="button.icon"
                :class="`icon ${button.customColour || 'fill-gray-700'}`"
              />

              <p>{{ button.name }}</p>
            </button>
          </div>

          <div v-else-if="button.type === ButtonType.Divider">
            <hr class="my-1 h-0.5 border-t-0 bg-neutral-100" />
          </div>
        </div>
      </li>
    </ul>
  </div>
</template>

<style scoped>
.item {
  @apply flex items-center pl-2 py-1 my-1 pr-16 w-full font-medium rounded-md hover:bg-gray-100;
}

.icon {
  @apply h-4 pr-2;
}
</style>

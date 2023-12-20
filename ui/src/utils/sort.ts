import type { FileT } from '@/lib/comet/types'

enum SortBy {
  NameAsc = 1,
  NameDesc = 2,
  UpdatedAsc = 3,
  UpdatedDesc = 4
}

const sortFiles = (items: FileT[], by: SortBy | string = SortBy.NameAsc): FileT[] => {
  if (typeof by === 'string') {
    by = SortBy[by as keyof typeof SortBy]
  }

  return items.sort((a, b) => {
    if (a.file_type === 'FOLDER') return -1
    if (b.file_type === 'FOLDER') return 1

    switch (by) {
      case SortBy.NameAsc:
        return a.name.localeCompare(b.name)
      case SortBy.NameDesc:
        return b.name.localeCompare(a.name)
      case SortBy.UpdatedAsc:
        return b.last_updated - a.last_updated
      case SortBy.UpdatedDesc:
        return a.last_updated - b.last_updated
      default:
        return 0
    }
  })
}

const setSortType = (sortType: SortBy): SortBy => {
  localStorage.setItem('sortType', sortType.toString())

  return sortType
}

export { SortBy, sortFiles, setSortType }


enum SortBy {
    NameAsc = 1,
    NameDesc = 2,
    UpdatedAsc = 3,
    UpdatedDesc = 4,
}

const sortFiles = <T extends { file_name: string; last_updated: number }>(
    arr: T[],
    by: SortBy | string = SortBy.NameAsc,
): T[] => {
    if (typeof by === "string") { // this is jank but enums are even jankier. :(
        by = SortBy[by as keyof typeof SortBy];
    }

    return arr.sort((a, b) => {
        switch (by) {
            case SortBy.NameAsc:
                return a.file_name.localeCompare(b.file_name);
            case SortBy.NameDesc:
                return b.file_name.localeCompare(a.file_name);
            case SortBy.UpdatedAsc:
                return b.last_updated - a.last_updated;
            case SortBy.UpdatedDesc:
                return a.last_updated - b.last_updated;
            default:
                return 0;
        }
    });
};

const setSortType = (sortType: SortBy): SortBy => {
    localStorage.setItem("sortType", sortType.toString());

    return sortType;
}


export {
    SortBy,
    sortFiles,
    setSortType
}
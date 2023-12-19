type Option<T> = T | null;

enum RequestType {
    QueryFolder = 'query_folder',

    // TODO: implement mesh network
    UserJoinedMesh = 'user_joined_mesh',
    UserLeftMesh = 'user_left_mesh',
    FileSendRequest = 'file_send_request',
    FileReceiveRequest = 'file_receive_request',
}

interface BaseRequestResponse {
    event: RequestType;
    request_id: string;
}

interface RequestFileResponse extends BaseRequestResponse {
    files: FileT[];
    folders: FolderT[];
}

interface FileT {
    original_file_name: Option<string>;
    file_name: string;
    file_ext: Option<string>;
    folder_id: Option<number>;
    file_id: number;
    last_updated: number;
}

interface FolderT {
    folder_name: string;
    folder_id: number;
    last_updated: Option<number>;
}

export type {
    FileT,
    FolderT,
    RequestFileResponse,
}
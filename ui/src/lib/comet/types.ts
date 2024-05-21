type Option<T> = T | null

enum RequestType {
  QueryFolder = 'query_folder',

  // TODO: implement mesh network
  UserJoinedMesh = 'user_joined_mesh',
  UserLeftMesh = 'user_left_mesh',
  FileSendRequest = 'file_send_request',
  FileReceiveRequest = 'file_receive_request'
}

interface BaseRequestResponse {
  event: RequestType
  request_id: string
}

interface RequestFileResponse extends BaseRequestResponse {
  files: FileT[]
}

interface FileT {
  id: number
  name: string
  file_type: FileType
  last_updated: number
}

enum FileType {
  FILE = 1,
  FOLDER = 2
}

export type { FileT, RequestFileResponse }
export { FileType }

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
  file_type: 'FILE' | 'FOLDER'
  last_updated: number
}

export type { FileT, RequestFileResponse }

export enum Role {
  Admin = 1,
  User = 1,
}

type WebSocketMessage<T, D> = {
  type: T;
  data: D;
};

export type DashboardStats = WebSocketMessage<
  "DashboardStats",
  {
    total_files: number;
    total_files_trend: number;

    storage_used_bytes: number;
    storage_used_bytes_trend: number;

    average_file_size_bytes: number;
    views: number;
  }
>;

type MediaItem = {
  media_id: string;
  file_url: string;
  file_size: number;
  uploaded_at: number;
  content_type: string;
  original_file_name: string;
};

export type UploadsList = WebSocketMessage<
  "UploadsList",
  {
    items: MediaItem[];
    next_cursor: string | null;
  }
>;

export type FileUpload = WebSocketMessage<"FileUpload", MediaItem>;
export type FileDelete = WebSocketMessage<"FileDelete", string>;

export type ExifHeaders = {
  camera?: string;
  datetaken?: string;
  resolution?: string;
  aperture?: string;
  shutterspeed?: string;
  iso?: string;
  focallength?: string;
  flash?: string;
  whitebalance?: string;

  gpslatitude?: string;
  gpslongitude?: string;

  title?: string;
  artist?: string;
  album?: string;
};

import type { ExifHeaders, FilesList, Settings } from "./types";
import { BASE_URL, TOKEN_NAME } from "./utils";

class ApiClient {
  #baseUrl: string = BASE_URL;

  async #request(endpoint: string, options: RequestInit = {}) {
    const token = localStorage.getItem(TOKEN_NAME);

    const headers: Record<string, any> = {
      ...options.headers,
    };

    if (token) {
      headers.Authorization = `Bearer ${token}`;
    }

    if (options.body && !headers["Content-Type"]) {
      headers["Content-Type"] = "application/json";
    }

    const config = {
      ...options,
      headers,
    };

    try {
      const response = await fetch(`${this.#baseUrl}${endpoint}`, config);

      if (response.status === 401) {
        localStorage.removeItem(TOKEN_NAME);
      }

      return response;
    } catch (error) {
      console.error(`[api] error on: ${endpoint}:`, error);
      throw error;
    }
  }

  async checkAnyUserExists(): Promise<boolean> {
    const response = await this.#request("/does-any-user-exist");

    return response.json();
  }

  async login(values: { email: string; password: string }) {
    return this.#request("/login", {
      method: "POST",
      body: JSON.stringify(values),
    });
  }

  async register(values: { name: string; email: string; password: string }) {
    return this.#request("/register", {
      method: "POST",
      body: JSON.stringify(values),
    });
  }

  async getFiles({
    cursor,
    limit,
  }: {
    cursor: string | null;
    limit: number | null;
  }): Promise<FilesList> {
    const params = new URLSearchParams();
    if (cursor) params.append("cursor", cursor);
    if (limit) params.append("limit", limit.toString());

    const req = await this.#request(`/files?${params}`);
    return req.json();
  }

  async deleteFile(media_id: string) {
    return this.#request(`/view/${media_id}`, {
      method: "DELETE",
    });
  }

  async getFileMetadata(file_url: string): Promise<ExifHeaders> {
    const response = await this.#request(file_url, {
      method: "HEAD",
    });

    const exif: Record<string, string> = {};

    response.headers.forEach((value, key) => {
      if (key.startsWith("x-exif")) {
        exif[key.slice(7)] = value;
      } else if (key.startsWith("x-audio")) {
        exif[key.slice(8)] = value;
      }
    });

    return exif;
  }

  async getSettings(): Promise<Settings> {
    const response = await this.#request("/settings", {
      method: "GET",
    });

    return response.json();
  }

  async updateSetting(key: string, newValue: any) {
    return this.#request("/settings", {
      method: "PATCH",
      body: JSON.stringify({ [key]: newValue }),
    });
  }

  async resetPassword(current_password: string, new_password: string) {
    return this.#request("/profile/reset-password", {
      method: "POST",
      body: JSON.stringify({ current_password, new_password }),
    });
  }

  async resetKey() {
    return this.#request("/profile/reset-key", {
      method: "POST",
    });
  }
}

export const api = new ApiClient();

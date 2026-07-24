import { clsx, type ClassValue } from "clsx";
import { twMerge } from "tailwind-merge";

import { NavigateOptions, To, useNavigate } from "react-router-dom";
import { useEffect, useState } from "react";

// @ts-ignore
export const BASE_URL = process.env.APP_API_URL;
// @ts-ignore
export const WS_BASE_URL = process.env.APP_WS_URL;

export const TOKEN_NAME = "auth_token";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export function getAbsoluteUrl(pathOrUrl: string): string {
  if (pathOrUrl.startsWith("http://") || pathOrUrl.startsWith("https://")) {
    return pathOrUrl;
  }

  const cleanPath = pathOrUrl.startsWith("/") ? pathOrUrl : `/${pathOrUrl}`;
  return `${window.location.origin}${cleanPath}`;
}

export function formatBytes(bytes: number, decimals: number = 1): string {
  if (bytes === 0) return "0 Bytes";

  const k = 1024;
  const sizes = ["Bytes", "KB", "MB", "GB", "TB", "PB"];

  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return `${parseFloat((bytes / Math.pow(k, i)).toFixed(decimals))} ${sizes[i]}`;
}

export function useTransitionNavigate() {
  const navigate = useNavigate();

  return (to: To, options?: NavigateOptions) => {
    if (!document.startViewTransition) {
      navigate(to, options);
      return;
    }

    document.startViewTransition(() => {
      navigate(to, options);
    });
  };
}

export function debounce<T extends (...args: any[]) => void>(
  func: T,
  delay: number,
): (...args: Parameters<T>) => void {
  let timeoutId: ReturnType<typeof setTimeout> | null = null;

  return (...args: Parameters<T>) => {
    if (timeoutId !== null) clearTimeout(timeoutId);

    timeoutId = setTimeout(() => func(...args), delay);
  };
}

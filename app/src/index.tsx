import { createRoot } from "react-dom/client";
import { StrictMode } from "react";

// @ts-ignore
import "./global.css";

import { createBrowserRouter, RouterProvider } from "react-router-dom";

import { WebSocketProvider } from "./providers/WebSocketProvider";
import ProtectedLayout from "./providers/ProtectedLayout";

import Home from "./routes/Home";
import Dashboard from "./routes/Dashboard";
import Gallery from "./routes/Gallery";

const router = createBrowserRouter([
  {
    path: "/",
    element: <Home />,
  },
  {
    element: <ProtectedLayout />,
    children: [
      {
        path: "/dashboard",
        element: <Dashboard />,
      },
      {
        path: "/gallery",
        element: <Gallery />,
      },
    ],
  },
]);

let container = document.getElementById("app")!;
let root = createRoot(container);

root.render(
  <StrictMode>
    <WebSocketProvider>
      <RouterProvider router={router} />
    </WebSocketProvider>
  </StrictMode>,
);

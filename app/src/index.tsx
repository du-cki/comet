import { createRoot } from "react-dom/client";
import { StrictMode } from "react";

// @ts-ignore
import "./global.css";

import { createBrowserRouter, RouterProvider } from "react-router";
import { Home } from "./routes/Home";

let container = document.getElementById("app")!;
let root = createRoot(container);

// const BASE_URL = process.env.APP_API_URL;
// console.log(BASE);

const router = createBrowserRouter([
  {
    path: "/",
    element: <Home />,
  },
]);

root.render(
  <StrictMode>
    <RouterProvider router={router} />
  </StrictMode>,
);

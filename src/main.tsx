import { StrictMode } from "react";
import ReactDOM from "react-dom/client";
import { RouterProvider, createRouter } from "@tanstack/react-router";
import "@fontsource-variable/noto-sans-sc";
import { NextUIProvider } from "@nextui-org/react";

// Import the generated route tree
import { routeTree } from "./routeTree.gen";

import "./main.css";

// Create a new router instance
const router = createRouter({ routeTree });

// Register the router instance for type safety
declare module "@tanstack/react-router" {
  interface Register {
    router: typeof router;
  }
}

// Render the app
const rootElement = document.getElementById("root")!;
if (!rootElement.innerHTML) {
  const root = ReactDOM.createRoot(rootElement);
  root.render(
    <StrictMode>
      <NextUIProvider id="next-provider">
        <RouterProvider router={router} />
      </NextUIProvider>
    </StrictMode>
  );
}

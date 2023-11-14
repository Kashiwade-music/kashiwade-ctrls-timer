import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "@mantine/core/styles.css";
import { MantineProvider, DEFAULT_THEME } from "@mantine/core";
ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <MantineProvider theme={DEFAULT_THEME}>
    <App />
  </MantineProvider>
);

import { useState, useEffect } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import { appWindow } from "@tauri-apps/api/window";
import { emit, listen } from "@tauri-apps/api/event";
import { Sun, Moon } from "lucide-react";
import classes from "./App.module.css";

import {
  AppShell,
  ActionIcon,
  CloseButton,
  NativeSelect,
  Switch,
  Button,
  useMantineColorScheme,
  useMantineTheme,
} from "@mantine/core";
await appWindow.setAlwaysOnTop(true);

function App() {
  const [timerText, setTimerText] = useState("00:00");
  const [shouldTimerReset, setShouldTimerReset] = useState(false);
  const { colorScheme, setColorScheme, clearColorScheme, toggleColorScheme } =
    useMantineColorScheme({ keepTransitions: true });

  useEffect(() => {
    invoke("get_is_dark_mode").then((isDarkMode) => {
      setColorScheme(isDarkMode ? "dark" : "light");
    });
    console.log("out of useEffect, colorScheme", colorScheme);

    let unlistenSetTimer: any;
    let unlistenShouldTimerReset: any;
    async function f() {
      unlistenSetTimer = await listen("set-timer", (event) => {
        setTimerText(event.payload as string);
      });
      unlistenShouldTimerReset = await listen("should-timer-reset", (event) => {
        setShouldTimerReset(event.payload as boolean);
        setTimerText("00:00");
      });
    }
    f();

    return () => {
      if (unlistenSetTimer) {
        unlistenSetTimer();
      }
      if (unlistenShouldTimerReset) {
        unlistenShouldTimerReset();
      }
    };
  }, []);

  const sunIcon = <Sun size={17} />;
  const moonIcon = <Moon size={17} />;
  return (
    <AppShell header={{ height: 60 }}>
      <AppShell.Header>
        <div className={classes.header} data-tauri-drag-region>
          <Switch
            size="md"
            checked={colorScheme !== "dark"}
            color={useMantineTheme().colors.yellow[4]}
            onLabel={sunIcon}
            offLabel={moonIcon}
            onChange={() => {
              toggleColorScheme();
              invoke("set_is_dark_mode", {
                isDarkMode: colorScheme !== "dark",
              });
            }}
          />
          <CloseButton />
        </div>
      </AppShell.Header>
      <AppShell.Main>
        <div className="container">
          <p>{timerText}</p>
        </div>
      </AppShell.Main>
    </AppShell>
  );
}

export default App;

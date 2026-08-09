"use client";

import { Moon, Sun, Monitor } from "lucide-react";
import { Button } from "@/components/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { useTheme } from "../../lib/theme-context";
import { CommandShortcut } from "../command-provider/command-shortcut";

export default function ThemeToggle() {
  const { theme, setTheme, mounted } = useTheme();

  const getIcon = () => {
    if (!mounted) {
      // Return a neutral icon during hydration - use the theme state directly since it's available
      if (theme === "system") {
        return <Monitor className="h-[1.2rem] w-[1.2rem]" />;
      }
      if (theme === "light") {
        return <Sun className="h-[1.2rem] w-[1.2rem]" />;
      }
      if (theme === "dark") {
        return <Moon className="h-[1.2rem] w-[1.2rem]" />;
      }
      return <Monitor className="h-[1.2rem] w-[1.2rem]" />;
    }

    if (theme === "system") {
      return <Monitor className="h-[1.2rem] w-[1.2rem]" />;
    }
    if (theme === "light") {
      return <Sun className="h-[1.2rem] w-[1.2rem]" />;
    }
    if (theme === "dark") {
      return <Moon className="h-[1.2rem] w-[1.2rem]" />;
    }
    // Fallback should not be needed now
    return <Monitor className="h-[1.2rem] w-[1.2rem]" />;
  };

  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <Button variant="outline" size="icon" aria-label="Toggle theme">
          {getIcon()}
          <span className="sr-only">Toggle theme</span>
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent align="end">
        <DropdownMenuItem onClick={() => setTheme("light")}>
          <Sun className="mr-2 h-4 w-4" />
          <span>Light</span>
          <CommandShortcut
            commandId="light"
            commandLabel="Light"
            commandDescription="Set theme to light"
            commandIcon={<Sun className="mr-2 h-4 w-4" />}
            commandGroup="Theme"
            onExecute={() => {
              setTheme("light");
            }}
            shortcutKeys={["alt", "x"]}
          />
        </DropdownMenuItem>
        <DropdownMenuItem onClick={() => setTheme("dark")}>
          <Moon className="mr-2 h-4 w-4" />
          <span>Dark</span>
          <CommandShortcut
            commandId="dark"
            commandLabel="Dark"
            commandDescription="Set theme to dark"
            commandIcon={<Moon className="mr-2 h-4 w-4" />}
            commandGroup="Theme"
            onExecute={() => {
              setTheme("dark");
            }}
            shortcutKeys={["alt", "y"]}
          />
        </DropdownMenuItem>
        <DropdownMenuItem onClick={() => setTheme("system")}>
          <Monitor className="mr-2 h-4 w-4" />
          <span>System</span>
          <CommandShortcut
            commandId="system"
            commandLabel="System"
            commandDescription="Set theme to system"
            commandIcon={<Monitor className="mr-2 h-4 w-4" />}
            commandGroup="Theme"
            onExecute={() => {
              setTheme("system");
            }}
            shortcutKeys={["alt", "z"]}
          />
        </DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenu>
  );
}

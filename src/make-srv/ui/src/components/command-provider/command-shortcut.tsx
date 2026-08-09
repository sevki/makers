"use client";

import * as React from "react";
import { DropdownMenuShortcut } from "@/components/ui/dropdown-menu";
import { useRegisterCommand } from "./command-provider";
import { KbdCombo } from "../kbd/kbd";

export interface CommandShortcutProps {
  children?: React.ReactNode;
  commandId: string;
  commandLabel: string;
  commandDescription?: string;
  commandIcon?: React.ReactNode;
  commandGroup?: string;
  onExecute: () => void;
  shortcutKeys: string[]; // e.g., ["shift", "meta", "p"] for ⇧⌘P
}

function parseShortcutKeys(keys: string[]): string {
  const isMac =
    typeof navigator !== "undefined" &&
    /(Mac|iPhone|iPod|iPad)/i.test(navigator.platform);

  const keyMap: Record<string, string> = {
    shift: "⇧",
    meta: isMac ? "⌘" : "Ctrl",
    cmd: isMac ? "⌘" : "Ctrl",
    ctrl: isMac ? "⌃" : "Ctrl",
    alt: isMac ? "⌥" : "Alt",
    option: isMac ? "⌥" : "Alt",
  };

  return keys
    .map((key) => {
      const normalized = key.toLowerCase();
      return keyMap[normalized] || key.toUpperCase();
    })
    .join(isMac ? "" : "+");
}

function createKeyboardHandler(keys: string[], callback: () => void) {
  return (e: KeyboardEvent) => {
    const modifiers = {
      shift: e.shiftKey,
      meta: e.metaKey,
      cmd: e.metaKey,
      ctrl: e.ctrlKey,
      alt: e.altKey,
      option: e.altKey,
    };

    // Check if all required modifiers are pressed
    const requiredModifiers = keys.filter((k) => k.toLowerCase() in modifiers);
    const allModifiersPressed = requiredModifiers.every(
      (mod) => modifiers[mod.toLowerCase() as keyof typeof modifiers],
    );

    if (!allModifiersPressed) return;

    // Get the main key (non-modifier)
    const mainKey = keys.find((k) => !(k.toLowerCase() in modifiers));
    if (!mainKey) return;

    // Check if the main key matches
    if (e.key.toLowerCase() === mainKey.toLowerCase()) {
      e.preventDefault();
      callback();
    }
  };
}

export function CommandShortcut({
  commandId,
  commandLabel,
  commandDescription,
  commandIcon,
  commandGroup,
  onExecute,
  shortcutKeys,
}: CommandShortcutProps) {
  const shortcutDisplay = parseShortcutKeys(shortcutKeys);

  // Register the command with the provider
  try {
    useRegisterCommand({
      id: commandId,
      label: commandLabel,
      description: commandDescription,
      icon: commandIcon,
      shortcut: shortcutDisplay,
      action: onExecute,
      group: commandGroup,
    });

    // Register global keyboard shortcut
    React.useEffect(() => {
      const handler = createKeyboardHandler(shortcutKeys, onExecute);
      document.addEventListener("keydown", handler);

      return () => {
        document.removeEventListener("keydown", handler);
      };
    }, [shortcutKeys, onExecute]);
  } catch (error) {
    // If not within CommandProvider, just show the shortcut without registration
    console.warn("CommandShortcut used outside of CommandProvider context");
  }

  return (
    <DropdownMenuShortcut>
      <KbdCombo size={"sm"} variant="light" keys={shortcutKeys}></KbdCombo>
    </DropdownMenuShortcut>
  );
}

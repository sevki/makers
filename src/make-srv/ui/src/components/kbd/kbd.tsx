import "./kbd.css";
import * as React from "react";
import { cn } from "../../lib/utils";
import { useTheme } from "../../lib/theme-context";

export interface KbdProps extends React.HTMLAttributes<HTMLElement> {
  children: React.ReactNode;
  variant?: "default" | "light" | "dark";
  size?: "sm" | "md" | "lg";
}

function getPlatformKey(key: string): string {
  if (typeof navigator !== "undefined") {
    const isMac = /(Mac|iPhone|iPod|iPad)/i.test(navigator.platform);

    const keyMap: Record<string, string> = {
      meta: isMac ? "⌘" : "Ctrl",
      cmd: isMac ? "⌘" : "Ctrl",
      ctrl: isMac ? "⌃" : "Ctrl",
      alt: isMac ? "⌥" : "Alt",
      option: isMac ? "⌥" : "Alt",
      shift: "⇧",
      enter: "↵",
      return: "↵",
      delete: "⌫",
      backspace: "⌫",
      escape: "Esc",
      tab: "⇥",
      space: "␣",
      up: "↑",
      down: "↓",
      left: "←",
      right: "→",
    };

    const normalized = key.toLowerCase();
    return keyMap[normalized] || key;
  }
  return key;
}

export function Kbd({
  children,
  className,
  variant,
  size = "md",
  ...props
}: KbdProps) {
  const themeContext = useTheme();
  const processedChildren = React.useMemo(() => {
    if (typeof children === "string") {
      return getPlatformKey(children);
    }
    return children;
  }, [children]);

  return (
    <kbd
      className={cn(
        "kbd",
        `kbd-${variant ? variant : themeContext.theme}`,
        `kbd-${size}`,
        className,
      )}
      {...props}
    >
      {processedChildren}
    </kbd>
  );
}

// Compound component for key combinations
export function KbdCombo({
  keys,
  separator = "+",
  ...props
}: {
  keys: string[];
  separator?: string | React.ReactNode;
} & Omit<KbdProps, "children">) {
  return (
    <span className="kbd-combo">
      {keys.map((key, index) => (
        <React.Fragment key={index}>
          <Kbd {...props}>{key}</Kbd>
          {index < keys.length - 1 && (
            <span className="kbd-separator">{separator}</span>
          )}
        </React.Fragment>
      ))}
    </span>
  );
}

// Example usage component
export default function KbdDemo() {
  return (
    <div className="kbd-demo">
      <h2 className="kbd-demo-title">Keyboard Key Component</h2>

      <div className="kbd-demo-section">
        <h3>Single Keys</h3>
        <div className="kbd-demo-row">
          <Kbd>A</Kbd>
          <Kbd>Enter</Kbd>
          <Kbd>Escape</Kbd>
          <Kbd>Space</Kbd>
          <Kbd>Tab</Kbd>
        </div>
      </div>

      <div className="kbd-demo-section">
        <h3>Modifier Keys</h3>
        <div className="kbd-demo-row">
          <Kbd>Cmd</Kbd>
          <Kbd>Ctrl</Kbd>
          <Kbd>Alt</Kbd>
          <Kbd>Shift</Kbd>
          <Kbd>Option</Kbd>
        </div>
      </div>

      <div className="kbd-demo-section">
        <h3>Key Combinations</h3>
        <div className="kbd-demo-row">
          <KbdCombo keys={["Cmd", "K"]} />
          <KbdCombo keys={["Ctrl", "Shift", "P"]} />
          <KbdCombo keys={["Alt", "Tab"]} />
        </div>
      </div>

      <div className="kbd-demo-section">
        <h3>Sizes</h3>
        <div className="kbd-demo-row">
          <Kbd size="sm">Small</Kbd>
          <Kbd size="md">Medium</Kbd>
          <Kbd size="lg">Large</Kbd>
        </div>
      </div>

      <div className="kbd-demo-section">
        <h3>Variants</h3>
        <div className="kbd-demo-row">
          <Kbd variant="default">Default</Kbd>
          <Kbd variant="light">Light</Kbd>
          <Kbd variant="dark">Dark</Kbd>
        </div>
      </div>
    </div>
  );
}

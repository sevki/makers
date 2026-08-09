"use client";

import * as React from "react";

export interface CommandOption {
  id: string;
  label: string;
  description?: string;
  icon?: React.ReactNode;
  shortcut?: string;
  action: () => void;
  group?: string;
  keywords?: string[];
}

interface CommandContextValue {
  registerCommand: (command: CommandOption) => void;
  unregisterCommand: (id: string) => void;
  executeCommand: (id: string) => void;
  commands: CommandOption[];
  isOpen: boolean;
  setIsOpen: (open: boolean) => void;
}

const CommandContext = React.createContext<CommandContextValue | undefined>(
  undefined,
);

export function useCommandProvider() {
  const context = React.useContext(CommandContext);
  if (!context) {
    throw new Error("useCommandProvider must be used within CommandProvider");
  }
  return context;
}

export interface CommandProviderProps {
  children: React.ReactNode;
  defaultCommands?: CommandOption[];
  openKey?: string; // Default: "k"
  /**
   * Whether this instance listens for the global openKey shortcut. Set to
   * false for isolated/nested providers (e.g. component previews) so they
   * don't also pop open whenever the page's real command palette shortcut
   * is pressed elsewhere. @default true
   */
  enableShortcut?: boolean;
}

export function CommandProvider({
  children,
  defaultCommands = [],
  openKey = "/",
  enableShortcut = true,
}: CommandProviderProps) {
  const [commands, setCommands] =
    React.useState<CommandOption[]>(defaultCommands);
  const [isOpen, setIsOpen] = React.useState(false);

  const registerCommand = React.useCallback((command: CommandOption) => {
    setCommands((prev) => {
      // Prevent duplicate IDs
      const filtered = prev.filter((c) => c.id !== command.id);
      return [...filtered, command];
    });
  }, []);

  const unregisterCommand = React.useCallback((id: string) => {
    setCommands((prev) => prev.filter((c) => c.id !== id));
  }, []);

  const executeCommand = React.useCallback(
    (id: string) => {
      const command = commands.find((c) => c.id === id);
      if (command) {
        command.action();
        setIsOpen(false);
      }
    },
    [commands],
  );

  // Keyboard shortcut to open command palette
  React.useEffect(() => {
    if (!enableShortcut) return;

    const down = (e: KeyboardEvent) => {
      // For "/" key, don't require modifier, but don't trigger if typing in an input
      const isTyping =
        e.target instanceof HTMLInputElement ||
        e.target instanceof HTMLTextAreaElement ||
        (e.target instanceof HTMLElement && e.target.isContentEditable);

      if (openKey === "/") {
        if (e.key === "/" && !isTyping) {
          e.preventDefault();
          setIsOpen((open) => !open);
        }
      } else if (e.key === openKey && (e.metaKey || e.ctrlKey)) {
        e.preventDefault();
        setIsOpen((open) => !open);
      }
    };

    document.addEventListener("keydown", down);
    return () => document.removeEventListener("keydown", down);
  }, [openKey, enableShortcut]);

  return (
    <CommandContext.Provider
      value={{
        registerCommand,
        unregisterCommand,
        executeCommand,
        commands,
        isOpen,
        setIsOpen,
      }}
    >
      {children}
    </CommandContext.Provider>
  );
}

// Hook to register a command
export function useRegisterCommand(
  command: Omit<CommandOption, "action"> & { action: () => void },
  deps: React.DependencyList = [],
) {
  const { registerCommand, unregisterCommand } = useCommandProvider();

  React.useEffect(() => {
    registerCommand(command as CommandOption);
    return () => unregisterCommand(command.id);
  }, deps);
}

// Example usage component. Reuses an ambient CommandProvider if one is
// already mounted higher up the tree (as apps typically only want a single
// instance, since each CommandProvider attaches its own global "/" keydown
// listener); only falls back to mounting its own when none exists, so this
// demo doesn't fight a page-level provider for the shortcut.
function CommandProviderDemoContent() {
  const [count, setCount] = React.useState(0);
  const { setIsOpen } = useCommandProvider();

  return (
    <div className="space-y-4 p-4 text-center">
      <button
        type="button"
        className="text-sm text-muted-foreground underline"
        onClick={() => setIsOpen(true)}
      >
        Open command palette
      </button>
      <div>
        <p className="text-4xl font-bold">{count}</p>
        <button
          type="button"
          className="text-sm text-muted-foreground underline"
          onClick={() => setCount((c) => c + 1)}
        >
          Increment
        </button>
      </div>
    </div>
  );
}

export default function CommandProviderDemo() {
  let hasAmbientProvider = true;
  try {
    useCommandProvider();
  } catch {
    hasAmbientProvider = false;
  }

  if (hasAmbientProvider) {
    return <CommandProviderDemoContent />;
  }

  return (
    <CommandProvider defaultCommands={[]}>
      <CommandProviderDemoContent />
    </CommandProvider>
  );
}

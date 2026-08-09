import * as React from "react";
import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { api } from "@/lib/api";

interface NewJobDialogProps {
  onCreated?: () => void;
}

const initialForm = {
  repo: "",
  pr_number: "",
  head_sha: "",
  base_url: "http://localhost:4173",
  routes: "/",
};

export function NewJobDialog({ onCreated }: NewJobDialogProps) {
  const [open, setOpen] = React.useState(false);
  const [form, setForm] = React.useState(initialForm);
  const [submitting, setSubmitting] = React.useState(false);
  const [error, setError] = React.useState<string | null>(null);

  const handleSubmit = async (event: React.FormEvent) => {
    event.preventDefault();
    setSubmitting(true);
    setError(null);
    try {
      await api.createJob({
        repo: form.repo.trim(),
        pr_number: Number(form.pr_number),
        head_sha: form.head_sha.trim() || "unknown",
        base_url: form.base_url.trim(),
        routes: form.routes
          .split(",")
          .map((route) => route.trim())
          .filter(Boolean),
      });
      setForm(initialForm);
      setOpen(false);
      onCreated?.();
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setSubmitting(false);
    }
  };

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogTrigger asChild>
        <Button>New job</Button>
      </DialogTrigger>
      <DialogContent className="sm:max-w-md">
        <form onSubmit={handleSubmit}>
          <DialogHeader>
            <DialogTitle>Enqueue a screenshot job</DialogTitle>
            <DialogDescription>
              Captures the given routes from a running UI preview and posts them as a comment on
              the pull request.
            </DialogDescription>
          </DialogHeader>
          <div className="grid gap-4 py-4">
            <div className="grid gap-2">
              <Label htmlFor="repo">Repository</Label>
              <Input
                id="repo"
                placeholder="owner/name"
                required
                value={form.repo}
                onChange={(e) => setForm((f) => ({ ...f, repo: e.target.value }))}
              />
            </div>
            <div className="grid grid-cols-2 gap-4">
              <div className="grid gap-2">
                <Label htmlFor="pr_number">PR number</Label>
                <Input
                  id="pr_number"
                  type="number"
                  min={1}
                  required
                  value={form.pr_number}
                  onChange={(e) => setForm((f) => ({ ...f, pr_number: e.target.value }))}
                />
              </div>
              <div className="grid gap-2">
                <Label htmlFor="head_sha">Commit SHA</Label>
                <Input
                  id="head_sha"
                  placeholder="deadbeef"
                  value={form.head_sha}
                  onChange={(e) => setForm((f) => ({ ...f, head_sha: e.target.value }))}
                />
              </div>
            </div>
            <div className="grid gap-2">
              <Label htmlFor="base_url">Preview base URL</Label>
              <Input
                id="base_url"
                required
                value={form.base_url}
                onChange={(e) => setForm((f) => ({ ...f, base_url: e.target.value }))}
              />
            </div>
            <div className="grid gap-2">
              <Label htmlFor="routes">Routes (comma-separated)</Label>
              <Input
                id="routes"
                required
                value={form.routes}
                onChange={(e) => setForm((f) => ({ ...f, routes: e.target.value }))}
              />
            </div>
            {error && <p className="text-sm text-destructive">{error}</p>}
          </div>
          <DialogFooter>
            <Button type="submit" disabled={submitting}>
              {submitting ? "Submitting..." : "Enqueue job"}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
}

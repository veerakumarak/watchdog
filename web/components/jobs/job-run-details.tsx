"use client"

import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { StatusBadge } from "./status-badge"
import { LogTimeline } from "./log-timeline"
import { format, parseISO } from "date-fns"

interface RunDetail {
  jobId: string
  runId: string
  jobName: string
  startTime: string
  endTime: string
  duration: number
  status: "success" | "failed" | "running"
  logs: Array<{
    timestamp: string
    level: "info" | "warning" | "error"
    message: string
  }>
}

const mockRunDetails: RunDetail = {
  jobId: "1",
  runId: "run-001",
  jobName: "Database Backup",
  startTime: "2024-12-09T02:15:30Z",
  endTime: "2024-12-09T02:45:22Z",
  duration: 1792,
  status: "success",
  logs: [
    {
      timestamp: "2024-12-09T02:15:30Z",
      level: "info",
      message: "Starting database backup job",
    },
    {
      timestamp: "2024-12-09T02:15:31Z",
      level: "info",
      message: "Connected to database: prod-db-01",
    },
    {
      timestamp: "2024-12-09T02:15:45Z",
      level: "info",
      message: "Starting backup of table: users (1.2GB)",
    },
    {
      timestamp: "2024-12-09T02:25:12Z",
      level: "info",
      message: "Backup of users table completed",
    },
    {
      timestamp: "2024-12-09T02:25:13Z",
      level: "info",
      message: "Starting backup of table: orders (2.8GB)",
    },
    {
      timestamp: "2024-12-09T02:40:55Z",
      level: "info",
      message: "Backup of orders table completed",
    },
    {
      timestamp: "2024-12-09T02:40:56Z",
      level: "info",
      message: "Uploading backup to S3",
    },
    {
      timestamp: "2024-12-09T02:45:20Z",
      level: "info",
      message: "Backup uploaded successfully (4.0GB)",
    },
    {
      timestamp: "2024-12-09T02:45:22Z",
      level: "info",
      message: "Database backup job completed successfully",
    },
  ],
}

export function JobRunDetails({ jobId, runId }: { jobId: string; runId: string }) {
  const run = mockRunDetails

  const formatTime = (isoString: string) => {
    try {
      return format(parseISO(isoString), "PPpp")
    } catch {
      return isoString
    }
  }

  const formatDuration = (seconds: number) => {
    const hours = Math.floor(seconds / 3600)
    const minutes = Math.floor((seconds % 3600) / 60)
    const secs = seconds % 60

    if (hours > 0) return `${hours}h ${minutes}m ${secs}s`
    if (minutes > 0) return `${minutes}m ${secs}s`
    return `${secs}s`
  }

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-3xl font-bold text-foreground">{run.jobName}</h1>
        <p className="mt-2 text-muted-foreground">Run ID: {run.runId}</p>
      </div>

      <div className="grid gap-4 md:grid-cols-3">
        <Card className="border-border bg-card">
          <CardContent className="pt-6">
            <div className="space-y-2">
              <p className="text-sm font-medium text-muted-foreground">Status</p>
              <StatusBadge status={run.status} />
            </div>
          </CardContent>
        </Card>

        <Card className="border-border bg-card">
          <CardContent className="pt-6">
            <div className="space-y-2">
              <p className="text-sm font-medium text-muted-foreground">Duration</p>
              <p className="text-2xl font-bold text-foreground">{formatDuration(run.duration)}</p>
            </div>
          </CardContent>
        </Card>

        <Card className="border-border bg-card">
          <CardContent className="pt-6">
            <div className="space-y-2">
              <p className="text-sm font-medium text-muted-foreground">Execution Time</p>
              <p className="text-sm text-foreground">{formatTime(run.startTime)}</p>
            </div>
          </CardContent>
        </Card>
      </div>

      <Card className="border-border bg-card">
        <CardHeader>
          <CardTitle className="text-card-foreground">Timeline & Logs</CardTitle>
          <CardDescription>Execution logs and events</CardDescription>
        </CardHeader>
        <CardContent>
          <LogTimeline logs={run.logs} />
        </CardContent>
      </Card>
    </div>
  )
}

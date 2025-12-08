"use client"

import { format, parseISO } from "date-fns"
import { AlertCircle, Info } from "lucide-react"

interface LogEntry {
  timestamp: string
  level: "info" | "warning" | "error"
  message: string
}

interface LogTimelineProps {
  logs: LogEntry[]
}

export function LogTimeline({ logs }: LogTimelineProps) {
  const getLogIcon = (level: string) => {
    switch (level) {
      case "error":
        return <AlertCircle className="h-5 w-5 text-destructive" />
      case "warning":
        return <AlertCircle className="h-5 w-5 text-yellow-500" />
      case "info":
      default:
        return <Info className="h-5 w-5 text-chart-2" />
    }
  }

  const getLogColor = (level: string) => {
    switch (level) {
      case "error":
        return "border-destructive/30 bg-destructive/5"
      case "warning":
        return "border-yellow-500/30 bg-yellow-500/5"
      case "info":
      default:
        return "border-muted bg-muted/30"
    }
  }

  const formatTime = (isoString: string) => {
    try {
      return format(parseISO(isoString), "HH:mm:ss")
    } catch {
      return isoString
    }
  }

  return (
    <div className="space-y-3">
      {logs.map((log, index) => (
        <div key={index} className={`border rounded-lg p-3 ${getLogColor(log.level)}`}>
          <div className="flex gap-3">
            <div className="flex-shrink-0 pt-0.5">{getLogIcon(log.level)}</div>
            <div className="flex-1 min-w-0">
              <div className="flex items-center justify-between gap-2">
                <p className="font-mono text-sm text-muted-foreground">{formatTime(log.timestamp)}</p>
                <span className="inline-flex items-center px-2 py-1 rounded text-xs font-medium uppercase tracking-wider">
                  {log.level === "error" && "text-destructive"}
                  {log.level === "warning" && "text-yellow-600"}
                  {log.level === "info" && "text-chart-2"}
                </span>
              </div>
              <p className="mt-1 text-sm text-foreground break-words">{log.message}</p>
            </div>
          </div>
        </div>
      ))}
    </div>
  )
}

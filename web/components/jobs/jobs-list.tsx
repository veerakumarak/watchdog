"use client"

import { useState, useMemo } from "react"
import Link from "next/link"
import { Card } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { StatusBadge } from "./status-badge"
import { MoreHorizontal, Trash2, Edit2, Eye } from "lucide-react"
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger } from "@/components/ui/dropdown-menu"

interface Job {
  id: string
  name: string
  description: string
  schedule: string
  status: "active" | "paused" | "failed"
  lastRun: string
  nextRun: string
  successRate: number
  totalRuns: number
}

const mockJobs: Job[] = [
  {
    id: "1",
    name: "Database Backup",
    description: "Daily backup of production database",
    schedule: "0 2 * * *",
    status: "active",
    lastRun: "2024-12-09 02:15:30",
    nextRun: "2024-12-10 02:00:00",
    successRate: 98,
    totalRuns: 150,
  },
  {
    id: "2",
    name: "Email Notifications",
    description: "Send weekly digest emails",
    schedule: "0 9 * * MON",
    status: "active",
    lastRun: "2024-12-09 09:05:12",
    nextRun: "2024-12-16 09:00:00",
    successRate: 100,
    totalRuns: 52,
  },
  {
    id: "3",
    name: "Cache Cleanup",
    description: "Clean expired cache entries",
    schedule: "*/30 * * * *",
    status: "active",
    lastRun: "2024-12-09 13:30:45",
    nextRun: "2024-12-09 14:00:00",
    successRate: 95,
    totalRuns: 8900,
  },
  {
    id: "4",
    name: "Report Generation",
    description: "Generate monthly analytics reports",
    schedule: "0 0 1 * *",
    status: "paused",
    lastRun: "2024-11-01 00:45:22",
    nextRun: "N/A",
    successRate: 92,
    totalRuns: 12,
  },
]

export function JobsList() {
  const [jobs, setJobs] = useState<Job[]>(mockJobs)

  const jobsWithSuccessBars = useMemo(() => {
    return jobs.map((job) => ({
      ...job,
      failureRate: 100 - job.successRate,
    }))
  }, [jobs])

  const handleDelete = (id: string) => {
    setJobs(jobs.filter((job) => job.id !== id))
  }

  return (
    <div className="space-y-4">
      {jobsWithSuccessBars.map((job) => (
        <Card key={job.id} className="overflow-hidden border-border bg-card hover:shadow-lg transition-shadow">
          <div className="p-6">
            <div className="flex items-start justify-between">
              <div className="flex-1">
                <div className="flex items-center gap-3">
                  <h3 className="text-xl font-semibold text-card-foreground">{job.name}</h3>
                  <StatusBadge status={job.status} />
                </div>
                <p className="mt-1 text-sm text-muted-foreground">{job.description}</p>
              </div>
              <DropdownMenu>
                <DropdownMenuTrigger asChild>
                  <Button variant="ghost" size="icon" className="h-8 w-8">
                    <MoreHorizontal className="h-4 w-4" />
                  </Button>
                </DropdownMenuTrigger>
                <DropdownMenuContent align="end">
                  <DropdownMenuItem asChild>
                    <Link href={`/jobs/${job.id}/edit`} className="flex items-center gap-2 cursor-pointer">
                      <Edit2 className="h-4 w-4" />
                      Edit
                    </Link>
                  </DropdownMenuItem>
                  <DropdownMenuItem onClick={() => handleDelete(job.id)} className="text-destructive cursor-pointer">
                    <Trash2 className="h-4 w-4" />
                    Delete
                  </DropdownMenuItem>
                </DropdownMenuContent>
              </DropdownMenu>
            </div>

            <div className="mt-4 grid grid-cols-2 gap-4 md:grid-cols-4">
              <div>
                <p className="text-xs font-medium text-muted-foreground">Schedule</p>
                <p className="mt-1 font-mono text-sm text-foreground">{job.schedule}</p>
              </div>
              <div>
                <p className="text-xs font-medium text-muted-foreground">Last Run</p>
                <p className="mt-1 text-sm text-foreground">{job.lastRun}</p>
              </div>
              <div>
                <p className="text-xs font-medium text-muted-foreground">Next Run</p>
                <p className="mt-1 text-sm text-foreground">{job.nextRun}</p>
              </div>
              <div>
                <p className="text-xs font-medium text-muted-foreground">Total Runs</p>
                <p className="mt-1 text-sm text-foreground">{job.totalRuns}</p>
              </div>
            </div>

            <div className="mt-4">
              <div className="flex items-center justify-between mb-2">
                <p className="text-xs font-medium text-muted-foreground">Success Rate</p>
                <p className="text-sm font-semibold text-foreground">{job.successRate}%</p>
              </div>
              <div className="h-2 overflow-hidden rounded-full bg-muted">
                <div
                  className="h-full bg-gradient-to-r from-chart-1 to-chart-2"
                  style={{ width: `${job.successRate}%` }}
                />
              </div>
            </div>

            <div className="mt-4 flex gap-2">
              <Link href={`/jobs/${job.id}/runs/latest`}>
                <Button variant="outline" size="sm" className="gap-2 bg-transparent">
                  <Eye className="h-4 w-4" />
                  View Latest Run
                </Button>
              </Link>
            </div>
          </div>
        </Card>
      ))}

      {jobs.length === 0 && (
        <Card className="border-border bg-card">
          <div className="flex flex-col items-center justify-center p-12 text-center">
            <p className="text-muted-foreground">No jobs configured yet</p>
            <Link href="/jobs/create">
              <Button className="mt-4 bg-primary text-primary-foreground hover:bg-primary/90">
                Create Your First Job
              </Button>
            </Link>
          </div>
        </Card>
      )}
    </div>
  )
}

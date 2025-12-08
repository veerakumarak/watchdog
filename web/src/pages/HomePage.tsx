"use client"

import { useNavigate } from "react-router-dom"
import { Button } from "@/components/ui/button"
import { JobsList } from "@/components/jobs/jobs-list"

export function HomePage() {
  const navigate = useNavigate()

  return (
    <main className="flex-1 overflow-auto">
      <div className="p-8">
        <div className="mb-8 flex items-center justify-between">
          <div>
            <h1 className="text-3xl font-bold text-foreground">Job Monitoring</h1>
            <p className="mt-2 text-muted-foreground">Monitor and manage your scheduled jobs</p>
          </div>
          <Button
            onClick={() => navigate("/jobs/create")}
            className="bg-primary text-primary-foreground hover:bg-primary/90"
          >
            Create Job
          </Button>
        </div>

        <JobsList />
      </div>
    </main>
  )
}

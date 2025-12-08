"use client"

import { useParams, useNavigate } from "react-router-dom"
import { Button } from "@/components/ui/button"
import { JobRunDetails } from "@/components/jobs/job-run-details"
import { ChevronLeft } from "lucide-react"

export function JobRunDetailsPage() {
  const { id: jobId, runId } = useParams()
  const navigate = useNavigate()

  return (
    <main className="flex-1 overflow-auto">
      <div className="p-8">
        <Button variant="ghost" onClick={() => navigate("/")} className="mb-4 gap-2">
          <ChevronLeft className="h-4 w-4" />
          Back to Jobs
        </Button>

        <JobRunDetails jobId={jobId!} runId={runId!} />
      </div>
    </main>
  )
}

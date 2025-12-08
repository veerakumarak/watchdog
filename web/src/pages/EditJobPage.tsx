"use client"

import { useNavigate, useParams } from "react-router-dom"
import { JobForm } from "@/components/jobs/job-form"

export function EditJobPage() {
  const { id } = useParams()
  const navigate = useNavigate()

  return (
    <main className="flex-1 overflow-auto">
      <div className="p-8">
        <div className="mb-8">
          <h1 className="text-3xl font-bold text-foreground">Edit Job</h1>
          <p className="mt-2 text-muted-foreground">Update job configuration</p>
        </div>

        <JobForm jobId={id} isEditing onSubmit={() => navigate("/")} />
      </div>
    </main>
  )
}

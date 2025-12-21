"use client"

import { useNavigate } from "react-router-dom"
import { JobForm } from "@/components/jobs/job-form.tsx"

export function CreateJobPage() {
    const navigate = useNavigate()

    return (
        <main className="flex-1 overflow-auto">
            <div className="p-8">
                <div className="mb-8">
                    <h1 className="text-3xl font-bold text-foreground">Create Job</h1>
                    <p className="mt-2 text-muted-foreground">Configure a new scheduled job</p>
                </div>

                <JobForm onSubmit={() => navigate("/")} />
            </div>
        </main>
    )
}

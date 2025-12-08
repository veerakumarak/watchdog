import { JobForm } from "@/components/jobs/job-form"
import Link from "next/link"
import { Button } from "@/components/ui/button"
import { ChevronLeft } from "lucide-react"

export default function CreateJobPage() {
  return (
    <main className="flex-1 overflow-auto">
      <div className="p-8">
        <Link href="/">
          <Button variant="ghost" className="mb-6">
            <ChevronLeft className="mr-2 h-4 w-4" />
            Back to Jobs
          </Button>
        </Link>

        <div className="mb-8">
          <h1 className="text-3xl font-bold text-foreground">Create New Job</h1>
          <p className="mt-2 text-muted-foreground">Configure a new scheduled job</p>
        </div>

        <JobForm />
      </div>
    </main>
  )
}

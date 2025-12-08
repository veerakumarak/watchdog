import { JobRunDetails } from "@/components/jobs/job-run-details"
import Link from "next/link"
import { Button } from "@/components/ui/button"
import { ChevronLeft } from "lucide-react"

export function generateStaticParams() {
  return [
    { id: "job-1", runId: "run-1" },
    { id: "job-1", runId: "run-2" },
    { id: "job-2", runId: "run-1" },
  ]
}

export default async function JobRunDetailsPage({
  params,
}: {
  params: { id: string; runId: string }
}) {
  const { id, runId } = params

  return (
    <main className="flex-1 overflow-auto">
      <div className="p-8">
        <Link href="/">
          <Button variant="ghost" className="mb-6">
            <ChevronLeft className="mr-2 h-4 w-4" />
            Back to Jobs
          </Button>
        </Link>

        <JobRunDetails jobId={id} runId={runId} />
      </div>
    </main>
  )
}

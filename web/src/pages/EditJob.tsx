"use client"

import { useState, useEffect } from "react"
import { useParams, useNavigate } from "react-router-dom"
import JobForm from "../components/JobForm"
import "../styles/job-form.css"

export default function EditJob() {
  const { id } = useParams()
  const navigate = useNavigate()
  const [initialData, setInitialData] = useState(null)

  useEffect(() => {
    // Mock data fetching
    const mockJob = {
      name: "Database Backup",
      description: "Daily database backup to S3",
      schedule: "0 14 * * *",
      timeout: 3600,
      retries: 3,
      status: "active",
    }
    setInitialData(mockJob)
  }, [id])

  const handleSubmit = (jobData: any) => {
    console.log("Updating job:", id, jobData)
    // Here you would make an API call to update the job
    navigate("/")
  }

  if (!initialData) {
    return <div className="loading">Loading...</div>
  }

  return (
    <div className="job-form-page">
      <div className="form-header">
        <h1>Edit Job</h1>
        <p>Update job configuration</p>
      </div>
      <JobForm onSubmit={handleSubmit} initialData={initialData} isEditing={true} />
    </div>
  )
}

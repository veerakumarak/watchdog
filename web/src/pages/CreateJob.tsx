"use client"
import { useNavigate } from "react-router-dom"
import JobForm from "../components/JobForm"
import "../styles/job-form.css"

export default function CreateJob() {
  const navigate = useNavigate()

  const handleSubmit = (jobData: any) => {
    console.log("Creating job:", jobData)
    // Here you would make an API call to create the job
    navigate("/")
  }

  return (
    <div className="job-form-page">
      <div className="form-header">
        <h1>Create New Job</h1>
        <p>Configure a new scheduled job</p>
      </div>
      <JobForm onSubmit={handleSubmit} />
    </div>
  )
}

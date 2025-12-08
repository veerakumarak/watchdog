"use client"

import type React from "react"

import { useState } from "react"
import "../styles/job-form.css"

interface JobFormProps {
  onSubmit: (data: any) => void
  initialData?: any
  isEditing?: boolean
}

export default function JobForm({ onSubmit, initialData, isEditing }: JobFormProps) {
  const [formData, setFormData] = useState(
    initialData || {
      name: "",
      description: "",
      schedule: "0 14 * * *",
      timeout: 3600,
      retries: 3,
      status: "active",
    },
  )

  const handleChange = (e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement | HTMLSelectElement>) => {
    const { name, value } = e.target
    setFormData((prev) => ({
      ...prev,
      [name]: name === "timeout" || name === "retries" ? Number.parseInt(value) : value,
    }))
  }

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault()
    onSubmit(formData)
  }

  return (
    <form className="job-form" onSubmit={handleSubmit}>
      <div className="form-group">
        <label>Job Name</label>
        <input
          type="text"
          name="name"
          value={formData.name}
          onChange={handleChange}
          placeholder="e.g., Database Backup"
          required
        />
      </div>

      <div className="form-group">
        <label>Description</label>
        <textarea
          name="description"
          value={formData.description}
          onChange={handleChange}
          placeholder="What does this job do?"
          rows={3}
        />
      </div>

      <div className="form-row">
        <div className="form-group">
          <label>Cron Schedule</label>
          <input
            type="text"
            name="schedule"
            value={formData.schedule}
            onChange={handleChange}
            placeholder="0 14 * * *"
          />
          <small>Format: minute hour day month weekday</small>
        </div>

        <div className="form-group">
          <label>Timeout (seconds)</label>
          <input type="number" name="timeout" value={formData.timeout} onChange={handleChange} min="60" />
        </div>

        <div className="form-group">
          <label>Retries</label>
          <input type="number" name="retries" value={formData.retries} onChange={handleChange} min="0" max="5" />
        </div>
      </div>

      <div className="form-group">
        <label>Status</label>
        <select name="status" value={formData.status} onChange={handleChange}>
          <option value="active">Active</option>
          <option value="inactive">Inactive</option>
        </select>
      </div>

      <div className="form-actions">
        <button type="submit" className="btn btn-primary">
          {isEditing ? "Update Job" : "Create Job"}
        </button>
      </div>
    </form>
  )
}

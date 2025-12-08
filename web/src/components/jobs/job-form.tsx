"use client"

import type React from "react"

import { useState } from "react"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { Textarea } from "@/components/ui/textarea"
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select"

interface JobFormProps {
  jobId?: string
  isEditing?: boolean
  onSubmit?: () => void
}

export function JobForm({ jobId, isEditing = false, onSubmit }: JobFormProps) {
  const [loading, setLoading] = useState(false)
  const [formData, setFormData] = useState({
    name: isEditing ? "Database Backup" : "",
    description: isEditing ? "Daily backup of production database" : "",
    schedule: isEditing ? "0 2 * * *" : "0 0 * * *",
    timeout: isEditing ? "3600" : "1800",
    retries: isEditing ? "3" : "2",
    handler: isEditing ? "handlers.backupDatabase" : "",
  })

  const handleChange = (e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement | HTMLSelectElement>) => {
    const { name, value } = e.target
    setFormData((prev) => ({ ...prev, [name]: value }))
  }

  const handleSelectChange = (value: string) => {
    setFormData((prev) => ({ ...prev, schedule: value }))
  }

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setLoading(true)

    try {
      await new Promise((resolve) => setTimeout(resolve, 1000))
      console.log("Submitting job:", formData)
      onSubmit?.()
    } catch (error) {
      console.error("Error saving job:", error)
    } finally {
      setLoading(false)
    }
  }

  const presetSchedules = [
    { label: "Every minute", value: "* * * * *" },
    { label: "Every hour", value: "0 * * * *" },
    { label: "Daily at 2 AM", value: "0 2 * * *" },
    { label: "Monday at 9 AM", value: "0 9 * * MON" },
    { label: "First day of month", value: "0 0 1 * *" },
    { label: "Custom", value: "custom" },
  ]

  return (
    <Card className="max-w-2xl border-border bg-card">
      <CardHeader>
        <CardTitle className="text-card-foreground">Job Configuration</CardTitle>
        <CardDescription>{isEditing ? "Update the job settings" : "Set up a new scheduled job"}</CardDescription>
      </CardHeader>
      <CardContent>
        <form onSubmit={handleSubmit} className="space-y-6">
          <div className="space-y-2">
            <Label htmlFor="name" className="text-foreground">
              Job Name
            </Label>
            <Input
              id="name"
              name="name"
              placeholder="e.g., Database Backup"
              value={formData.name}
              onChange={handleChange}
              required
              className="border-border bg-background text-foreground"
            />
          </div>

          <div className="space-y-2">
            <Label htmlFor="description" className="text-foreground">
              Description
            </Label>
            <Textarea
              id="description"
              name="description"
              placeholder="Describe what this job does"
              value={formData.description}
              onChange={handleChange}
              className="border-border bg-background text-foreground"
              rows={3}
            />
          </div>

          <div className="space-y-2">
            <Label htmlFor="handler" className="text-foreground">
              Handler Function
            </Label>
            <Input
              id="handler"
              name="handler"
              placeholder="e.g., handlers.processJob"
              value={formData.handler}
              onChange={handleChange}
              required
              className="border-border bg-background text-foreground"
            />
          </div>

          <div className="space-y-2">
            <Label htmlFor="schedule" className="text-foreground">
              Schedule (Cron)
            </Label>
            <Select value={formData.schedule} onValueChange={handleSelectChange}>
              <SelectTrigger className="border-border bg-background text-foreground">
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                {presetSchedules.map((preset) => (
                  <SelectItem key={preset.value} value={preset.value}>
                    {preset.label}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
            {formData.schedule === "custom" && (
              <Input
                name="customSchedule"
                placeholder="0 0 * * *"
                className="mt-2 border-border bg-background text-foreground"
              />
            )}
            <p className="text-xs text-muted-foreground">
              Enter a cron expression (e.g., "0 2 * * *" for daily at 2 AM)
            </p>
          </div>

          <div className="grid grid-cols-2 gap-4">
            <div className="space-y-2">
              <Label htmlFor="timeout" className="text-foreground">
                Timeout (seconds)
              </Label>
              <Input
                id="timeout"
                name="timeout"
                type="number"
                placeholder="1800"
                value={formData.timeout}
                onChange={handleChange}
                className="border-border bg-background text-foreground"
              />
            </div>

            <div className="space-y-2">
              <Label htmlFor="retries" className="text-foreground">
                Max Retries
              </Label>
              <Input
                id="retries"
                name="retries"
                type="number"
                placeholder="3"
                value={formData.retries}
                onChange={handleChange}
                className="border-border bg-background text-foreground"
              />
            </div>
          </div>

          <div className="flex gap-3 pt-4">
            <Button type="submit" disabled={loading} className="bg-primary text-primary-foreground hover:bg-primary/90">
              {loading ? "Saving..." : isEditing ? "Update Job" : "Create Job"}
            </Button>
          </div>
        </form>
      </CardContent>
    </Card>
  )
}

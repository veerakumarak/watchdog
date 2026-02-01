"use client"

import { Badge } from "@/components/ui/badge"
import { CheckCircle2, AlertCircle, Clock } from "lucide-react"

interface StatusBadgeProps {
    status: "active" | "paused" | "failed" | "success" | "running"
}

export function StatusBadge({ status }: StatusBadgeProps) {
    const statusConfig = {
        active: {
            icon: CheckCircle2,
            label: "Active",
            className: "bg-green-500/10 text-green-700 dark:text-green-400 border-green-500/30",
        },
        paused: {
            icon: Clock,
            label: "Paused",
            className: "bg-yellow-500/10 text-yellow-700 dark:text-yellow-400 border-yellow-500/30",
        },
        failed: {
            icon: AlertCircle,
            label: "Failed",
            className: "bg-red-500/10 text-red-700 dark:text-red-400 border-red-500/30",
        },
        success: {
            icon: CheckCircle2,
            label: "Success",
            className: "bg-green-500/10 text-green-700 dark:text-green-400 border-green-500/30",
        },
        running: {
            icon: Clock,
            label: "Running",
            className: "bg-blue-500/10 text-blue-700 dark:text-blue-400 border-blue-500/30",
        },
    }

    const config = statusConfig[status]
    const Icon = config.icon

    return (
        <Badge className={`gap-1.5 ${config.className} border`}>
            <Icon className="h-3 w-3" />
            {config.label}
        </Badge>
    )
}

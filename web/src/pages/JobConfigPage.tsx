import {
    Card,
    CardContent,
} from "@/components/ui/card";
import {Badge} from "@/components/ui/badge";
import {Switch} from "@/components/ui/switch";
import {
    Clock,
    Globe,
    MoreHorizontal,
    PlayCircle,
    CheckCircle2, Edit, Plus,
} from "lucide-react";
import {Button} from "@/components/ui/button";
import {ScrollArea, ScrollBar} from "@/components/ui/scroll-area";
import {Tooltip, TooltipContent, TooltipProvider, TooltipTrigger} from "@/components/ui/tooltip";

import {useEffect, useState} from "react";
import JobFormSheet from "@/components/sheets/job-form-sheet.tsx";
import type {JobConfig} from "@/lib/types.ts";
import {toast} from "sonner";
import {get, post, put} from "@/lib/fetcher.ts";
import {getId} from "@/lib/helpers.ts";

type JobsListResponse = {
    jobConfigs: JobConfig[];
}

// --- Component: Single Job Row ---
const JobRow = ({job}: { job: JobConfig }) => {
    return (
        <Card
            className="group mb-3 hover:shadow-md transition-all border-l-4 border-l-transparent hover:border-l-blue-600">
            <CardContent className="p-4">
                <div className="grid grid-cols-1 lg:grid-cols-12 gap-4 items-center">

                    {/* COLUMN 1: Identity (Span 3) */}
                    <div className="lg:col-span-3 flex flex-col gap-1">
                        <div className="flex items-center gap-2 mb-1">
                            <Badge variant="outline"
                                   className="text-[10px] uppercase tracking-wider text-blue-600 bg-blue-50 border-blue-100">
                                {job.appName}
                            </Badge>
                            {!job.enabled &&
                                <Badge variant="secondary" className="text-[10px] h-5 px-1">Disabled</Badge>}
                        </div>
                        <h3 className="font-bold text-gray-800 text-base truncate" title={job.jobName}>
                            {job.jobName}
                        </h3>
                    </div>

                    {/* COLUMN 2: Meta Data (Span 3) */}
                    <div
                        className="lg:col-span-3 flex flex-col gap-2 text-sm text-muted-foreground border-l border-r border-dashed px-4 border-gray-100">
                        <div className="flex items-center gap-2">
                            <Clock className="w-3.5 h-3.5 text-gray-400"/>
                            <code className="bg-slate-100 px-1.5 py-0.5 rounded text-xs font-mono text-slate-700">
                                {job.schedule}
                            </code>
                        </div>
                        <div className="flex items-center gap-2 text-xs">
                            <Globe className="w-3.5 h-3.5 text-gray-400"/>
                            <span>{job.zoneId}</span>
                        </div>
                    </div>

                    {/* COLUMN 3: Stages (Span 5) */}
                    <div className="lg:col-span-5 overflow-hidden">
                        <div className="text-[10px] uppercase font-semibold text-gray-400 mb-1 flex items-center gap-1">
                            Stages
                        </div>

                        {/* Horizontal Scroll Area for Stages */}
                        <ScrollArea className="w-full whitespace-nowrap pb-2">
                            <div className="flex gap-2">
                                {job.stages.map((stage, idx) => (
                                    <div
                                        key={idx}
                                        className="flex flex-col justify-center items-start bg-slate-50 border border-slate-100 rounded-md px-2 py-1 min-w-[80px]"
                                    >
                    <span className="text-[10px] font-bold text-slate-600 capitalize mb-1 truncate max-w-[70px]">
                      {stage.name}
                    </span>
                                        <div className="flex gap-1">
                                            {stage.start !== null && (
                                                <TooltipProvider>
                                                    <Tooltip>
                                                        <TooltipTrigger>
                            <span
                                className="flex items-center text-[10px] text-amber-600 bg-amber-50 px-1 rounded border border-amber-100">
                              <PlayCircle className="w-2 h-2 mr-0.5"/>{stage.start}
                            </span>
                                                        </TooltipTrigger>
                                                        <TooltipContent><p>Start Offset</p></TooltipContent>
                                                    </Tooltip>
                                                </TooltipProvider>
                                            )}
                                            {stage.complete !== null && (
                                                <TooltipProvider>
                                                    <Tooltip>
                                                        <TooltipTrigger>
                              <span
                                  className="flex items-center text-[10px] text-emerald-600 bg-emerald-50 px-1 rounded border border-emerald-100">
                                <CheckCircle2 className="w-2 h-2 mr-0.5"/>{stage.complete}
                              </span>
                                                        </TooltipTrigger>
                                                        <TooltipContent><p>Complete By</p></TooltipContent>
                                                    </Tooltip>
                                                </TooltipProvider>
                                            )}
                                        </div>
                                    </div>
                                ))}
                            </div>
                            <ScrollBar orientation="horizontal" className="h-1.5"/>
                        </ScrollArea>
                    </div>

                    {/* COLUMN 4: Actions (Span 1) */}
                    <div className="lg:col-span-1 flex lg:flex-col items-end justify-center gap-2 pl-2">
                        <Switch checked={job.enabled} className="scale-75 origin-right"/>
                        <Button variant="ghost" size="icon" className="h-8 w-8">
                            <MoreHorizontal className="w-4 h-4 text-gray-500"/>
                        </Button>
                    </div>

                </div>
            </CardContent>
        </Card>
    );
};


export const JobConfigPage = () => {
    // --- State for the Sheet ---
    const [sheetOpen, setSheetOpen] = useState(false);
    const [editingJob, setEditingJob] = useState<JobConfig | null>(null);

    // State for the list data
    const [jobs, setJobs] = useState<JobConfig[]>([]);
    const [loading, setLoading] = useState(true);

    // 1. Load Data on Mount
    useEffect(() => {
        loadJobs();
    }, []);

    const loadJobs = async () => {
        try {
            setLoading(true);
            const data = await get<JobsListResponse>('/job-configs');
            console.log('data is' + data);
            setJobs(data.get().jobConfigs);
        } catch (error) {
            console.error("Failed to load jobs", error);
            toast.error("Could not load jobs.");
        } finally {
            setLoading(false);
        }
    };

    // --- Actions ---

    // 1. Open for "Create"
    const handleCreate = () => {
        setEditingJob(null); // Ensure no data is passed
        setSheetOpen(true);
    };

    // 2. Open for "Edit"
    const handleEdit = (job: JobConfig) => {
        setEditingJob(job); // Pass the job data
        setSheetOpen(true);
    };

    // 3. Handle Form Submission
    const handleFormSubmit = async (data: JobConfig) => {
        try {
            if (editingJob) {
                // UPDATE MODE
                let result = await put<JobConfig, JobConfig>('/job-configs', data);
                console.log(result);
                // result = await JobService.update(data);
                // console.log(result);

                // Optimistic UI Update (or you can just call loadJobs())
                setJobs(prev => prev.map(j => (getId(j) === getId(data)) ? result.get() : j));

                toast.success("Job configuration updated.");
            } else {
                // CREATE MODE
                let result = await post<JobConfig, JobConfig>('/job-configs', data);
                console.log(result);
                // result = await JobService.create(data);
                // setJobs(prev => [...prev, result]);
                toast.success("New job created.");
            }

            setSheetOpen(false); // Close the sheet on success
        } catch (error) {
            console.error("Operation failed", error);
            toast.error(error instanceof Error ? error.message : "Unknown error");
            // Note: We do NOT close the sheet here, so user can fix their input
        }
    };

    if (loading) {
        return <div>Loading... please wait.</div>;
    }

    return (
        <div className="space-y-6">
            <div className="flex items-center justify-between">
                <div>
                    <h2 className="text-lg font-bold">Job Configurations</h2>
                    <p className="text-sm text-muted-foreground">Manage schedules, SLAs, and execution pipelines.</p>
                </div>
                {/*<div>*/}
                {/*    <h1 className="text-2xl font-bold tracking-tight text-slate-900">Job Configurations</h1>*/}
                {/*    <p className="text-muted-foreground">Manage schedules, SLAs, and execution pipelines.</p>*/}
                {/*</div>*/}
                {/*<h2 className="text-xl font-bold tracking-tight">Job Configurations</h2>*/}
                {/* CREATE BUTTON */}
                <Button onClick={handleCreate} className="bg-blue-600 hover:bg-blue-700" disabled={loading}>
                    <Plus className="w-4 h-4 mr-2"/> New Job
                </Button>
            </div>

            <div className="space-y-1">
                {/*<Button onClick={handleCreateClick} className="bg-blue-600 hover:bg-blue-700">*/}
                {/*    <Plus className="w-4 h-4 mr-2" /> New Job*/}
                {/*</Button>*/}

                {/*{mockJobs.map((job) => (*/}
                {/*    <JobRow key={job.id} job={job} />*/}
                {/*))}*/}
                {/*{loading ? (<div>Loading... please wait</div>): ""}*/}

                {!loading && jobs.map((job) => (
                    // Use your JobRow here, passing the edit handler
                    <div key={getId(job)} className="relative group">
                        {/* ... Your JobRow contents ... */}
                        <JobRow key={getId(job)} job={job}/>

                        {/* Example of where the Edit button lives in your JobRow */}
                        <div className="absolute top-4 right-14">
                            <Button
                                variant="ghost"
                                size="sm"
                                onClick={() => handleEdit(job)}
                            >
                                <Edit className="w-4 h-4 text-gray-500"/>
                            </Button>
                        </div>
                    </div>
                ))}
                {/*</div>*/}

                {/* THE SHARED SHEET COMPONENT */}
                <JobFormSheet
                    open={sheetOpen}
                    onOpenChange={setSheetOpen}
                    initialData={editingJob}
                    onSubmit={handleFormSubmit}
                />
            </div>
        </div>
    );
};

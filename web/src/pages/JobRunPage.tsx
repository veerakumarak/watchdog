import {useEffect, useState} from 'react';
import {
    Table, TableBody, TableCell, TableHead, TableHeader, TableRow
} from "@/components/ui/table";
import {
    Sheet, SheetContent, SheetHeader, SheetTitle, SheetDescription
} from "@/components/ui/sheet";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Eye, RotateCw, Filter } from "lucide-react";
import { format } from 'date-fns';
import type {JobRun} from '@/lib/types';
import RunTimeline from "@/components/jobs/RunTimeline";
import {get} from "@/lib/fetcher";
import {toast} from "sonner";

type JobRunsListResponse = {
    jobRuns: JobRun[];
}

const JobRunsPage = () => {
    const [selectedRun, setSelectedRun] = useState<JobRun | null>(null);

    const [runs, setRuns] = useState<JobRun[]>([]);
    const [loading, setLoading] = useState(true);

    useEffect(() => {
        loadRuns();
    }, []);

    const loadRuns = async () => {
        try {
            setLoading(true);
            const data = await get<JobRunsListResponse>('/job-runs');
            console.log('data is' + data);
            setRuns(data.get().jobRuns);
        } catch (error) {
            console.error("Failed to load job runs", error);
            toast.error("Could not load job runs.");
        } finally {
            setLoading(false);
        }
    };

    const getStatusBadge = (status: string) => {
        switch (status) {
            case 'Complete': return <Badge className="bg-emerald-600 hover:bg-emerald-700">Success</Badge>;
            case 'Failed': return <Badge className="bg-red-500 hover:bg-red-600">Failed</Badge>;
            case 'InProgress': return <Badge className="bg-blue-600 hover:bg-blue-700 animate-pulse">Running</Badge>;
            default: return <Badge variant="secondary">{status}</Badge>;
        }
    };

    if (loading) {
        return <div>Loading... please wait.</div>;
    }

    return (
        <div className="space-y-6">
            {/* Header */}
            <div className="flex items-center justify-between">
                <div>
                    <h2 className="text-lg font-bold">Execution History</h2>
                    <p className="text-sm text-muted-foreground">Monitor job runs and stage-level SLAs.</p>
                </div>

                {/*<div>*/}
                {/*    <h1 className="text-2xl font-bold tracking-tight text-slate-900">Execution History</h1>*/}
                {/*    <p className="text-muted-foreground">Monitor job runs and stage-level SLAs.</p>*/}
                {/*</div>*/}
                <div className="flex gap-2">
                    <Button variant="outline" size="sm"><Filter className="w-4 h-4 mr-2"/> Filter</Button>
                    <Button variant="outline" size="sm"><RotateCw className="w-4 h-4 mr-2"/> Refresh</Button>
                </div>
            </div>

            {/* Main Table */}
            <div className="rounded-md border bg-white shadow-sm">
                <Table>
                    <TableHeader>
                        <TableRow className="bg-slate-50">
                            <TableHead>Triggered At</TableHead>
                            <TableHead>Application</TableHead>
                            <TableHead>Job Name</TableHead>
                            <TableHead>Status</TableHead>
                            <TableHead className="text-right">Actions</TableHead>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        {runs.map((run) => (
                            <TableRow key={run.id} className="cursor-pointer hover:bg-slate-50" onClick={() => setSelectedRun(run)}>
                                <TableCell className="font-mono text-xs">
                                    {format(new Date(run.triggeredAt), 'yyyy-MM-dd HH:mm:ss')}
                                </TableCell>
                                <TableCell><Badge variant="outline">{run.appName}</Badge></TableCell>
                                <TableCell className="font-medium">{run.jobName}</TableCell>
                                <TableCell>{getStatusBadge(run.status)}</TableCell>
                                <TableCell className="text-right">
                                    <Button variant="ghost" size="sm" onClick={(e) => { e.stopPropagation(); setSelectedRun(run); }}>
                                        <Eye className="w-4 h-4 text-slate-500" />
                                    </Button>
                                </TableCell>
                            </TableRow>
                        ))}
                    </TableBody>
                </Table>
            </div>

            {/* Detail Slide-Over */}
            <Sheet open={!!selectedRun} onOpenChange={(open) => !open && setSelectedRun(null)}>
                <SheetContent className="w-[400px] sm:w-[540px] overflow-y-auto bg-white">
                    {selectedRun && (
                        <>
                            <SheetHeader className="mb-6 pb-6 border-b">
                                <div className="flex items-center gap-2 mb-2">
                                    {getStatusBadge(selectedRun.status)}
                                    <span className="text-xs font-mono text-muted-foreground">{selectedRun.id.slice(0, 8)}...</span>
                                </div>
                                <SheetTitle className="text-xl">{selectedRun.jobName}</SheetTitle>
                                <SheetDescription>
                                    Application: <span className="font-medium text-foreground">{selectedRun.appName}</span>
                                </SheetDescription>
                            </SheetHeader>

                            <div className="space-y-6">
                                {/* Timestamps */}
                                <div className="grid grid-cols-2 gap-4 text-sm bg-slate-50 p-4 rounded-lg">
                                    <div>
                                        <span className="text-muted-foreground text-xs block">Triggered</span>
                                        <span className="font-mono">{format(new Date(selectedRun.triggeredAt), 'PP p')}</span>
                                    </div>
                                    <div>
                                        <span className="text-muted-foreground text-xs block">Last Update</span>
                                        <span className="font-mono">{format(new Date(selectedRun.updatedAt), 'PP p')}</span>
                                    </div>
                                </div>

                                {/* Stages Timeline */}
                                <div>
                                    <h3 className="text-sm font-medium text-muted-foreground uppercase tracking-wider mb-4">
                                        Stage Execution
                                    </h3>
                                    <RunTimeline stages={selectedRun.stages} />
                                </div>
                            </div>
                        </>
                    )}
                </SheetContent>
            </Sheet>
        </div>
    );
};

export default JobRunsPage;
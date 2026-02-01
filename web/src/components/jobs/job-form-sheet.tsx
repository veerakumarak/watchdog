import {useEffect, useState} from 'react';
import {
    Sheet,
    SheetContent,
    SheetHeader,
    SheetTitle,
    SheetDescription,
    SheetFooter,
} from "@/components/ui/sheet";
import {Button} from "@/components/ui/button";
import {Input} from "@/components/ui/input";
import {Label} from "@/components/ui/label";
import {Switch} from "@/components/ui/switch";
import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue
} from "@/components/ui/select";
import {Separator} from "@/components/ui/separator";
import {Plus, Trash2, Save} from "lucide-react";
import type {JobConfig} from "@/lib/types.ts";

// --- Types ---
// Matches your JobConfig structure
// export type JobConfig = {
//     id?: string;
//     app_name: string;
//     job_name: string;
//     schedule: string;
//     zone_id: string;
//     channel_ids: string;
//     enabled: boolean;
//     stages: Stage[];
// };
//
export type Stage = {
    name: string;
    start: number | null;
    complete: number | null;
};

// Internal Form State (allows strings for empty inputs)
type StageInput = {
    name: string;
    start: string | number;
    complete: string | number;
};

interface JobFormSheetProps {
    open: boolean;
    onOpenChange: (open: boolean) => void;
    initialData: JobConfig | null; // If null -> Create Mode, If exists -> Edit Mode
    onSubmit: (data: JobConfig) => void;
}

const DEFAULT_FORM_STATE = {
    appName: "",
    jobName: "",
    schedule: "0 0 * * * *",
    zoneId: "UTC",
    channel_ids: "",
    enabled: true,
};

const DEFAULT_STAGES: StageInput[] = [{name: "dqa", start: "", complete: 0}];

const JobFormSheet = ({open, onOpenChange, initialData, onSubmit}: JobFormSheetProps) => {
    const isEditMode = !!initialData;

    // Form State
    const [formData, setFormData] = useState(DEFAULT_FORM_STATE);
    const [stages, setStages] = useState<StageInput[]>(DEFAULT_STAGES);

    // --- Effect: Reset or Populate Form on Open ---
    useEffect(() => {
        if (open) {
            if (initialData) {
                // EDIT MODE: Populate fields
                setFormData({
                    appName: initialData.appName,
                    jobName: initialData.jobName,
                    schedule: initialData.schedule,
                    zoneId: initialData.zoneId,
                    channel_ids: initialData.channel_ids,
                    enabled: initialData.enabled,
                });

                // Map stages (handle nulls -> empty strings for inputs)
                setStages(initialData.stages.map(s => ({
                    name: s.name,
                    start: s.start ?? "",
                    complete: s.complete ?? ""
                })));
            } else {
                // CREATE MODE: Reset to defaults
                setFormData(DEFAULT_FORM_STATE);
                setStages(DEFAULT_STAGES);
            }
        }
    }, [open, initialData]);

    // --- Handlers ---
    const handleInputChange = (field: string, value: any) => {
        setFormData(prev => ({...prev, [field]: value}));
    };

    const handleStageChange = (index: number, field: keyof StageInput, value: string) => {
        const newStages = [...stages];
        newStages[index] = {...newStages[index], [field]: value};
        setStages(newStages);
    };

    const addStage = () => {
        setStages([...stages, {name: "", start: "", complete: ""}]);
    };

    const removeStage = (index: number) => {
        setStages(stages.filter((_, i) => i !== index));
    };

    const handleSubmit = () => {
        // Convert form state back to strict types
        const cleanedStages: Stage[] = stages.map(s => ({
            name: s.name,
            start: s.start === "" ? null : Number(s.start),
            complete: s.complete === "" ? null : Number(s.complete)
        }));

        const payload: JobConfig = {
            ...formData,
            stages: cleanedStages,
        };

        onSubmit(payload);
        onOpenChange(false);
    };

    return (
        <Sheet open={open} onOpenChange={onOpenChange}>
            <SheetContent className="w-[400px] sm:w-[540px] overflow-y-auto bg-white border-l shadow-2xl">
                <SheetHeader className="mb-6">
                    <SheetTitle>{isEditMode ? "Edit Job Configuration" : "Create Job Configuration"}</SheetTitle>
                    <SheetDescription>
                        {isEditMode
                            ? `Modify settings for ${initialData?.jobName}. Identity fields are locked.`
                            : "Define the schedule, context, and SLA stages for the new job."}
                    </SheetDescription>
                </SheetHeader>

                <div className="space-y-6">

                    {/* SECTION 1: Identity (Locked in Edit Mode) */}
                    <div className="space-y-4">
                        <h3 className="text-sm font-medium text-muted-foreground uppercase tracking-wider">
                            Identity
                        </h3>
                        <div className="grid grid-cols-2 gap-4">
                            <div className="space-y-2">
                                <Label htmlFor="app">Application</Label>
                                <Input
                                    id="app"
                                    value={formData.appName}
                                    onChange={(e) => handleInputChange("app_name", e.target.value)}
                                    disabled={isEditMode} // Cannot change Primary Key
                                    className={isEditMode ? "bg-slate-100 text-slate-500" : ""}
                                />
                            </div>
                            <div className="space-y-2">
                                <Label htmlFor="job">Job Name</Label>
                                <Input
                                    id="job"
                                    value={formData.jobName}
                                    onChange={(e) => handleInputChange("job_name", e.target.value)}
                                    disabled={isEditMode} // Cannot change Primary Key
                                    className={isEditMode ? "bg-slate-100 text-slate-500" : ""}
                                />
                            </div>
                        </div>
                    </div>

                    {/* SECTION 2: Editable Config */}
                    <div className="space-y-4">
                        <h3 className="text-sm font-medium text-muted-foreground uppercase tracking-wider">
                            Settings
                        </h3>
                        <div className="grid grid-cols-2 gap-4">
                            <div className="space-y-2">
                                <Label htmlFor="schedule">Cron Schedule</Label>
                                <Input
                                    id="schedule"
                                    className="font-mono text-sm"
                                    value={formData.schedule}
                                    onChange={(e) => handleInputChange("schedule", e.target.value)}
                                />
                            </div>
                            <div className="space-y-2">
                                <Label htmlFor="timezone">Timezone</Label>
                                <Select
                                    value={formData.zoneId}
                                    onValueChange={(val) => handleInputChange("zone_id", val)}
                                >
                                    <SelectTrigger>
                                        <SelectValue placeholder="Select zone"/>
                                    </SelectTrigger>
                                    <SelectContent>
                                        <SelectItem value="UTC">UTC</SelectItem>
                                        <SelectItem value="US/Eastern">US/Eastern</SelectItem>
                                        <SelectItem value="US/Pacific">US/Pacific</SelectItem>
                                        <SelectItem value="Europe/London">Europe/London</SelectItem>
                                    </SelectContent>
                                </Select>
                            </div>
                        </div>

                        <div className="space-y-2">
                            <Label htmlFor="channels">Notification Channels</Label>
                            <Input
                                id="channels"
                                value={formData.channel_ids}
                                onChange={(e) => handleInputChange("channel_ids", e.target.value)}
                            />
                        </div>

                        <div className="flex items-center justify-between bg-slate-50 p-3 rounded-lg border">
                            <Label htmlFor="enabled" className="cursor-pointer">Job Enabled</Label>
                            <Switch
                                id="enabled"
                                checked={formData.enabled}
                                onCheckedChange={(checked) => handleInputChange("enabled", checked)}
                            />
                        </div>
                    </div>

                    <Separator/>

                    {/* SECTION 3: Stages */}
                    <div className="space-y-4">
                        <div className="flex items-center justify-between">
                            <h3 className="text-sm font-medium text-muted-foreground uppercase tracking-wider">
                                Stages
                            </h3>
                            <Button variant="outline" size="sm" onClick={addStage} className="h-8">
                                <Plus className="w-3 h-3 mr-1"/> Add Stage
                            </Button>
                        </div>

                        <div className="space-y-3">
                            {stages.map((stage, index) => (
                                <div key={index}
                                     className="flex gap-2 items-start p-3 rounded-md border border-slate-200 bg-slate-50/50 group hover:bg-slate-50 transition-colors">
                                    <div className="flex-grow space-y-1">
                                        <Label className="text-xs text-muted-foreground">Name</Label>
                                        <Input
                                            className="h-8 text-sm"
                                            value={stage.name}
                                            onChange={(e) => handleStageChange(index, "name", e.target.value)}
                                        />
                                    </div>
                                    <div className="w-20 space-y-1">
                                        <Label className="text-xs text-amber-600 flex items-center gap-1">Start</Label>
                                        <Input
                                            type="number"
                                            className="h-8 text-sm"
                                            value={stage.start}
                                            onChange={(e) => handleStageChange(index, "start", e.target.value)}
                                        />
                                    </div>
                                    <div className="w-20 space-y-1">
                                        <Label className="text-xs text-emerald-600 flex items-center gap-1">End</Label>
                                        <Input
                                            type="number"
                                            className="h-8 text-sm"
                                            value={stage.complete}
                                            onChange={(e) => handleStageChange(index, "complete", e.target.value)}
                                        />
                                    </div>
                                    <div className="pt-6">
                                        <Button
                                            variant="ghost"
                                            size="icon"
                                            className="h-8 w-8 text-gray-400 hover:text-red-600"
                                            onClick={() => removeStage(index)}
                                            disabled={stages.length === 1}
                                        >
                                            <Trash2 className="w-4 h-4"/>
                                        </Button>
                                    </div>
                                </div>
                            ))}
                        </div>
                    </div>
                </div>

                <SheetFooter className="mt-8">
                    <Button variant="outline" onClick={() => onOpenChange(false)}>Cancel</Button>
                    <Button onClick={handleSubmit} className="bg-blue-600 hover:bg-blue-700">
                        <Save className="w-4 h-4 mr-2"/> {isEditMode ? "Update Job" : "Create Job"}
                    </Button>
                </SheetFooter>
            </SheetContent>
        </Sheet>
    );
};

export default JobFormSheet;
import {
    Sheet,
    SheetContent,
    SheetHeader,
    SheetTitle,
    SheetDescription,
    SheetFooter,
    SheetTrigger
} from "@/components/ui/sheet";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Switch } from "@/components/ui/switch";
import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue
} from "@/components/ui/select";
import { Separator } from "@/components/ui/separator";
import { Plus, Trash2, Save, PlayCircle, CheckCircle2 } from "lucide-react";
import {useState} from "react";

// --- Types ---
type StageInput = {
    name: string;
    start: string | number; // String allowed for empty input handling
    complete: string | number;
};

const NewJobForm = () => {
    const [open, setOpen] = useState(false);

    // Form State
    const [formData, setFormData] = useState({
        app_name: "",
        job_name: "",
        schedule: "0 0 * * * *",
        zone_id: "UTC",
        channel_ids: "",
        enabled: true,
    });

    // Stages State (Dynamic Array)
    const [stages, setStages] = useState<StageInput[]>([
        { name: "dqa", start: "", complete: 0 }, // Default example row
    ]);

    // --- Handlers ---

    const handleInputChange = (field: string, value: any) => {
        setFormData(prev => ({ ...prev, [field]: value }));
    };

    const handleStageChange = (index: number, field: keyof StageInput, value: string) => {
        const newStages = [...stages];
        // Allow empty string for UI, convert to number if possible
        newStages[index] = { ...newStages[index], [field]: value };
        setStages(newStages);
    };

    const addStage = () => {
        setStages([...stages, { name: "", start: "", complete: "" }]);
    };

    const removeStage = (index: number) => {
        setStages(stages.filter((_, i) => i !== index));
    };

    const handleSubmit = () => {
        // 1. Clean up stages (convert empty strings to null for backend)
        const cleanedStages = stages.map(s => ({
            name: s.name,
            start: s.start === "" ? null : Number(s.start),
            complete: s.complete === "" ? null : Number(s.complete)
        }));

        const payload = {
            ...formData,
            stages: cleanedStages
        };

        console.log("Submitting Payload:", payload);
        setOpen(false);
        // TODO: Call your create_config API here
    };

    return (
        <Sheet open={open} onOpenChange={setOpen}>
            <SheetTrigger asChild>
                <Button className="bg-blue-600 hover:bg-blue-700">
                    <Plus className="w-4 h-4 mr-2" /> New Job
                </Button>
            </SheetTrigger>

            <SheetContent className="w-[400px] sm:w-[540px] overflow-y-auto bg-white">
                <SheetHeader className="mb-6">
                    <SheetTitle>Create Job Configuration</SheetTitle>
                    <SheetDescription>
                        Define the schedule, context, and SLA stages for the new job.
                    </SheetDescription>
                </SheetHeader>

                <div className="space-y-6">

                    {/* SECTION 1: Identity & Context */}
                    <div className="space-y-4">
                        <h3 className="text-sm font-medium text-muted-foreground uppercase tracking-wider">
                            General Info
                        </h3>

                        <div className="grid grid-cols-2 gap-4">
                            <div className="space-y-2">
                                <Label htmlFor="app">Application</Label>
                                <Input
                                    id="app"
                                    placeholder="e.g. gemini"
                                    value={formData.app_name}
                                    onChange={(e) => handleInputChange("app_name", e.target.value)}
                                />
                            </div>
                            <div className="space-y-2">
                                <Label htmlFor="job">Job Name</Label>
                                <Input
                                    id="job"
                                    placeholder="e.g. daily_ingest"
                                    value={formData.job_name}
                                    onChange={(e) => handleInputChange("job_name", e.target.value)}
                                />
                            </div>
                        </div>

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
                                    value={formData.zone_id}
                                    onValueChange={(val) => handleInputChange("zone_id", val)}
                                >
                                    <SelectTrigger>
                                        <SelectValue placeholder="Select zone" />
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
                                placeholder="e.g. gmail, slack (comma separated)"
                                value={formData.channel_ids}
                                onChange={(e) => handleInputChange("channel_ids", e.target.value)}
                            />
                        </div>

                        <div className="flex items-center justify-between bg-slate-50 p-3 rounded-lg border">
                            <Label htmlFor="enabled" className="cursor-pointer">Enable Job immediately?</Label>
                            <Switch
                                id="enabled"
                                checked={formData.enabled}
                                onCheckedChange={(checked) => handleInputChange("enabled", checked)}
                            />
                        </div>
                    </div>

                    <Separator />

                    {/* SECTION 2: Dynamic Stages */}
                    <div className="space-y-4">
                        <div className="flex items-center justify-between">
                            <h3 className="text-sm font-medium text-muted-foreground uppercase tracking-wider">
                                Stages Configuration
                            </h3>
                            <Button variant="outline" size="sm" onClick={addStage} className="h-8">
                                <Plus className="w-3 h-3 mr-1" /> Add Stage
                            </Button>
                        </div>

                        <div className="space-y-3">
                            {stages.map((stage, index) => (
                                <div key={index} className="flex gap-2 items-start p-3 rounded-md border border-slate-200 bg-slate-50/50 group hover:bg-slate-50 transition-colors">

                                    {/* Stage Name */}
                                    <div className="flex-grow space-y-1">
                                        <Label className="text-xs text-muted-foreground">Stage Name</Label>
                                        <Input
                                            placeholder="name"
                                            className="h-8 text-sm"
                                            value={stage.name}
                                            onChange={(e) => handleStageChange(index, "name", e.target.value)}
                                        />
                                    </div>

                                    {/* Start Offset */}
                                    <div className="w-20 space-y-1">
                                        <Label className="text-xs text-amber-600 flex items-center gap-1">
                                            <PlayCircle className="w-3 h-3" /> Start
                                        </Label>
                                        <Input
                                            type="number"
                                            placeholder="-"
                                            className="h-8 text-sm"
                                            value={stage.start}
                                            onChange={(e) => handleStageChange(index, "start", e.target.value)}
                                        />
                                    </div>

                                    {/* Complete Offset */}
                                    <div className="w-20 space-y-1">
                                        <Label className="text-xs text-emerald-600 flex items-center gap-1">
                                            <CheckCircle2 className="w-3 h-3" /> End
                                        </Label>
                                        <Input
                                            type="number"
                                            placeholder="-"
                                            className="h-8 text-sm"
                                            value={stage.complete}
                                            onChange={(e) => handleStageChange(index, "complete", e.target.value)}
                                        />
                                    </div>

                                    {/* Delete Button */}
                                    <div className="pt-6">
                                        <Button
                                            variant="ghost"
                                            size="icon"
                                            className="h-8 w-8 text-gray-400 hover:text-red-600"
                                            onClick={() => removeStage(index)}
                                            disabled={stages.length === 1} // Prevent deleting last row
                                        >
                                            <Trash2 className="w-4 h-4" />
                                        </Button>
                                    </div>
                                </div>
                            ))}
                        </div>

                        <div className="text-[10px] text-muted-foreground italic">
                            * Offsets are in minutes relative to schedule time. Leave empty if not applicable.
                        </div>
                    </div>
                </div>

                <SheetFooter className="mt-8">
                    <Button variant="outline" onClick={() => setOpen(false)}>Cancel</Button>
                    <Button onClick={handleSubmit} className="bg-blue-600 hover:bg-blue-700">
                        <Save className="w-4 h-4 mr-2" /> Save Config
                    </Button>
                </SheetFooter>

            </SheetContent>
        </Sheet>
    );
};

export default NewJobForm;
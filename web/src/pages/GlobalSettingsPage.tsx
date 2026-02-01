import { Card, CardContent, CardHeader, CardTitle, CardDescription, CardFooter } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
// import { Switch } from "@/components/ui/switch";
import {BellRing, Info, Save} from "lucide-react";
import {useEffect, useState} from "react";
// import {Checkbox} from "@radix-ui/react-checkbox";
// import {Separator} from "@/components/ui/separator.tsx";
import {MultiSelect} from "@/components/ui/multi-select.tsx";
import {get} from "@/lib/fetcher.ts";
import {toast} from "sonner";
import type {Channel, JobConfig} from "@/lib/types.ts";
import type {Stage} from "@/components/jobs/job-form-sheet.tsx";

type ChannelInfo = {
    label: string;
    value: string;
}
type ChannelsListResponse = {
    channels: Channel[];
}


const GlobalSettingsPage = () => {
    const [selectedChannels, setSelectedChannels] = useState<string[]>([]);

    const [channels, setChannels] = useState<ChannelInfo[]>([]);
    const [loading, setLoading] = useState(true);

    useEffect(() => {
        loadChannels();
    }, []);

    const loadChannels = async () => {
        setLoading(true);
        const data = await get<ChannelsListResponse>('/channels');
        console.log('data is' + data);
        if (data.isOk()) {
            setChannels(data.get().channels.map((channel: Channel) => ({label: channel.name, value: channel.providerType})));
        } else {
            console.error("Failed to load channels", data.failure());
            toast.error("Could not load channels." + data.failure().message);
        }
        setLoading(false);
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
        <div className="max-w-2xl space-y-6">
            <div>
                <h2 className="text-lg font-bold">Global Configuration</h2>
                <p className="text-sm text-muted-foreground">System-wide parameters for the watchdog service.</p>
            </div>

            {/* Data Retention Card */}
            <Card>
                <CardHeader>
                    <CardTitle>Data Retention</CardTitle>
                    <CardDescription>How long should job execution logs be kept?</CardDescription>
                </CardHeader>
                <CardContent className="space-y-4">
                    <div className="grid grid-cols-2 gap-4">
                        <div className="space-y-2">
                            <Label>Success Retention (Days)</Label>
                            <Input type="number" defaultValue={30} />
                        </div>
                        <div className="space-y-2">
                            <Label>Failure Retention (Days)</Label>
                            <Input type="number" defaultValue={90} />
                        </div>
                    </div>
                </CardContent>
            </Card>

            {/* System Defaults Card */}
            {/*<Card>*/}
            {/*    <CardHeader>*/}
            {/*        <CardTitle>System Defaults</CardTitle>*/}
            {/*    </CardHeader>*/}
            {/*    <CardContent className="space-y-4">*/}
            {/*        <div className="space-y-2">*/}
            {/*            <Label>Admin Email (Fallbacks)</Label>*/}
            {/*            <Input defaultValue="admin@company.com" />*/}
            {/*            <p className="text-[10px] text-muted-foreground">Used if no channel is specified for a critical job.</p>*/}
            {/*        </div>*/}

            {/*        <div className="flex items-center justify-between p-3 border rounded-lg bg-slate-50">*/}
            {/*            <div className="space-y-0.5">*/}
            {/*                <Label className="text-base">Maintenance Mode</Label>*/}
            {/*                <p className="text-xs text-muted-foreground">Pause all job scheduling immediately.</p>*/}
            {/*            </div>*/}
            {/*            <Switch />*/}
            {/*        </div>*/}
            {/*    </CardContent>*/}
            {/*    <CardFooter className="bg-slate-50/50 border-t p-4 flex justify-end">*/}
            {/*        <Button className="bg-blue-600 hover:bg-blue-700">*/}
            {/*            <Save className="w-4 h-4 mr-2" /> Save Global Settings*/}
            {/*        </Button>*/}
            {/*    </CardFooter>*/}
            {/*</Card>*/}

            <Card>
                <CardHeader>
                    <CardTitle>System Notifications & Defaults</CardTitle>
                    <CardDescription>Configure where system-level errors and job failures are sent.</CardDescription>
                </CardHeader>
                <CardContent className="space-y-6">
                    <div className="space-y-4">
                        <div className="space-y-2">
                            <Label className="flex items-center gap-2">
                                <BellRing className="w-4 h-4 text-blue-600" />
                                Default Alert Channels
                            </Label>

                            {/*<div className="grid grid-cols-2 gap-2 mt-2">*/}
                            {/*    {channelOptions.map((channel) => (*/}
                            {/*        <div key={channel.value} className="flex items-center space-x-2 p-2 border rounded-md hover:bg-slate-50">*/}
                            {/*            <Checkbox*/}
                            {/*                id={channel.value}*/}
                            {/*                checked={selectedChannels.includes(channel.value)}*/}
                            {/*                onCheckedChange={(checked) => {*/}
                            {/*                    if (checked) setSelectedChannels([...selectedChannels, channel.value])*/}
                            {/*                    else setSelectedChannels(selectedChannels.filter(c => c !== channel.value))*/}
                            {/*                }}*/}
                            {/*            />*/}
                            {/*            <label htmlFor={channel.value} className="text-sm cursor-pointer">{channel.label}</label>*/}
                            {/*        </div>*/}
                            {/*    ))}*/}
                            {/*</div>*/}

                             {/*The Multi-Select Input*/}
                            <MultiSelect
                                options={channels}
                                selected={selectedChannels}
                                onChange={setSelectedChannels}
                                placeholder="Select channels for system alerts..."
                            />
                            {/*<MultiSelect*/}
                            {/*    options={channelOptions}*/}
                            {/*    defaultValue={selectedChannels}*/}
                            {/*    onValueChange={setSelectedChannels}*/}
                            {/*    placeholder="Select notification channels"*/}
                            {/*    variant="inverted"*/}
                            {/*/>*/}

                            <p className="text-[11px] text-muted-foreground flex items-center gap-1">
                                <Info className="w-3 h-3" />
                                All job failures will be broadcast to these channels unless overridden.
                            </p>
                        </div>

                        {/*<Separator />*/}

                        {/*<div className="space-y-2">*/}
                        {/*    <Label>Admin Email (Fallbacks)</Label>*/}
                        {/*    <Input defaultValue="admin@company.com" />*/}
                        {/*    <p className="text-[10px] text-muted-foreground">Used for direct SMTP alerts if channels fail.</p>*/}
                        {/*</div>*/}
                    </div>

                    {/*<div className="flex items-center justify-between p-3 border rounded-lg bg-slate-50">*/}
                    {/*    <div className="space-y-0.5">*/}
                    {/*        <Label className="text-base">Maintenance Mode</Label>*/}
                    {/*        <p className="text-xs text-muted-foreground">Pause all job scheduling immediately.</p>*/}
                    {/*    </div>*/}
                    {/*    <Switch />*/}
                    {/*</div>*/}
                </CardContent>
                <CardFooter className="bg-slate-50/50 border-t p-4 flex justify-end">
                    <Button className="bg-blue-600 hover:bg-blue-700" onClick={handleSubmit}>
                        <Save className="w-4 h-4 mr-2" /> Save Global Settings
                    </Button>
                </CardFooter>
            </Card>
        </div>
    );
};

export default GlobalSettingsPage;
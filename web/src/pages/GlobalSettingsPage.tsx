import { Card, CardContent, CardHeader, CardTitle, CardDescription, CardFooter } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import {BellRing, Info, Save} from "lucide-react";
import {useEffect, useState} from "react";
import {MultiSelect} from "@/components/ui/multi-select";
import {get, post, put} from "@/lib/fetcher";
import {toast} from "sonner";
import type {Channel} from "@/lib/types";

type Settings = {
    successRetentionDays: number;
    failureRetentionDays: number;
    maintenanceMode: boolean;
    defaultChannels: String;
    errorChannels: String;
    maxStageDurationHours: number;
}

type SettingsResponse = {
    settings: Settings;
}

type ChannelsListResponse = {
    channels: Channel[];
}

const GlobalSettingsPage = () => {
    const [selectedDefaultChannels, setSelectedDefaultChannels] = useState<string[]>([]);
    const [selectedErrorChannels, setSelectedErrorChannels] = useState<string[]>([]);

    const [channels, setChannels] = useState<Channel[]>([]);
    const [settings, setSettings] = useState<Settings>(null);
    const [loading, setLoading] = useState(true);
    const [saving, setSaving] = useState(false); // New state for button loading

    useEffect(() => {
        const init = async () => {
            await Promise.all([loadChannels(), loadSettings()]);
        };
        init();
    }, []);

    const loadSettings = async () => {
        setLoading(true);
        const data = await get<SettingsResponse>('/settings');
        console.log('data is' + data);
        if (data.isOk()) {
            const s = data.get().settings;
            setSettings(s);
            setSelectedDefaultChannels(s.defaultChannels ? s.defaultChannels.split(',') : []);
            setSelectedErrorChannels(s.errorChannels ? s.errorChannels.split(',') : []);
        } else {
            console.error("Failed to load channels", data.failure());
            toast.error("Could not load channels." + data.failure().message);
        }
        setLoading(false);
    };

    const loadChannels = async () => {
        const data = await get<ChannelsListResponse>('/channels');
        if (data.isOk()) {
            setChannels(data.get().channels);
        } else {
            // console.error("Failed to load channels", data.failure());
            toast.error("Could not load channels." + data.failure().message);
        }
    };

    const handleSubmit = async () => {
        if (!settings) return;
        setSaving(true);

        // Prepare payload: join arrays back into comma-separated strings
        const payload: Settings = {
            ...settings,
            defaultChannels: selectedDefaultChannels.join(','),
            errorChannels: selectedErrorChannels.join(','),
        };

        const result = await put<Settings, SettingsResponse>('/settings', payload);

        if (result.isOk()) {
            toast.success("Settings updated successfully");
            setSettings(result.get().settings);
        } else {
            toast.error("Failed to save: " + result.failure().message);
        }
        setSaving(false);
        // onSubmit(payload);
    };

    // Helper to update numeric fields in the settings object
    const updateSettingField = (field: keyof Settings, value: string) => {
        if (!settings) return;
        setSettings({
            ...settings,
            [field]: value === "" ? 0 : Number(value)
        });
    };

    if (loading) {
        return <div className="p-8 text-center">Loading settings... please wait.</div>;
    }

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
                            <Input type="number" value={settings.successRetentionDays} onChange={(e) => updateSettingField('successRetentionDays', e.target.value)} />
                        </div>
                        <div className="space-y-2">
                            <Label>Failure Retention (Days)</Label>
                            <Input type="number" value={settings.failureRetentionDays} onChange={(e) => updateSettingField('failureRetentionDays', e.target.value)} />
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
                            {/*    {channels.map((channel) => (*/}
                            {/*        <div key={channel.name} className="flex items-center space-x-2 p-2 border rounded-md hover:bg-slate-50">*/}
                            {/*            <Checkbox*/}
                            {/*                id={channel.name}*/}
                            {/*                checked={selectedChannels.includes(channel.name)}*/}
                            {/*                onCheckedChange={(checked) => {*/}
                            {/*                    if (checked) setSelectedChannels([...selectedChannels, channel.name])*/}
                            {/*                    else setSelectedChannels(selectedChannels.filter(c => c !== channel.name))*/}
                            {/*                }}*/}
                            {/*            />*/}
                            {/*            <label htmlFor={channel.name} className="text-sm cursor-pointer">{channel.name}</label>*/}
                            {/*        </div>*/}
                            {/*    ))}*/}
                            {/*</div>*/}

                             {/*The Multi-Select Input*/}
                            <MultiSelect
                                options={channels.map(channel => ({
                                    label: channel.name,
                                    value: channel.name,
                                }))}
                                selected={selectedDefaultChannels}
                                onChange={setSelectedDefaultChannels}
                                placeholder="Select channels for system alerts..."
                                // variant="inverted"
                            />

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

                        <div className="space-y-4">
                            <Label className="flex items-center gap-2">
                                <BellRing className="w-4 h-4 text-blue-600" />
                                Default Error Channels
                            </Label>

                            {/*<div className="grid grid-cols-2 gap-2 mt-2">*/}
                            {/*    {channels.map((channel) => (*/}
                            {/*        <div key={channel.name} className="flex items-center space-x-2 p-2 border rounded-md hover:bg-slate-50">*/}
                            {/*            <Checkbox*/}
                            {/*                id={channel.name}*/}
                            {/*                checked={selectedChannels.includes(channel.name)}*/}
                            {/*                onCheckedChange={(checked) => {*/}
                            {/*                    if (checked) setSelectedChannels([...selectedChannels, channel.name])*/}
                            {/*                    else setSelectedChannels(selectedChannels.filter(c => c !== channel.name))*/}
                            {/*                }}*/}
                            {/*            />*/}
                            {/*            <label htmlFor={channel.name} className="text-sm cursor-pointer">{channel.name}</label>*/}
                            {/*        </div>*/}
                            {/*    ))}*/}
                            {/*</div>*/}

                            {/*The Multi-Select Input*/}
                            <MultiSelect
                                options={channels.map(channel => ({
                                    label: channel.name,
                                    value: channel.name,
                                }))}
                                selected={selectedErrorChannels}
                                onChange={setSelectedErrorChannels}
                                placeholder="Select channels for system errors..."
                                // variant="inverted"
                            />

                            <p className="text-[11px] text-muted-foreground flex items-center gap-1">
                                <Info className="w-3 h-3" />
                                All system errors will be broadcast to these channels unless overridden.
                            </p>
                        </div>


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
                    <Button className="bg-blue-600 hover:bg-blue-700" onClick={handleSubmit} disabled={saving} >
                        <Save className="w-4 h-4 mr-2" /> Save Global Settings
                    </Button>
                </CardFooter>
            </Card>
        </div>
    );
};

export default GlobalSettingsPage;
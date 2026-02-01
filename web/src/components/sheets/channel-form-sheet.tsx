import { useState, useEffect } from 'react';
import {
    Sheet, SheetContent, SheetHeader, SheetTitle, SheetDescription, SheetFooter
} from "@/components/ui/sheet.tsx";
import { Button } from "@/components/ui/button.tsx";
import { Input } from "@/components/ui/input.tsx";
import { Label } from "@/components/ui/label.tsx";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select.tsx";
import { Separator } from "@/components/ui/separator.tsx";
import { Save, Mail, MessageSquare/*, Hash */} from "lucide-react";
import type {Channel, ProviderType} from "@/lib/types.ts";
// import {ScrollArea, ScrollBar} from "@/components/ui/scroll-area.tsx";

interface ChannelFormSheetProps {
    open: boolean;
    onOpenChange: (open: boolean) => void;
    initialData: Channel | null;
    onSubmit: (data: Channel) => void;
}

export type ChannelForm = {
    id: string;
    name: string;
    providerType: ProviderType;
    configuration: Record<string, any>;
    createdAt?: string;
}

const toForm = (data: Channel): ChannelForm => {
    return {
        id: data.id,
        name: data.name,
        providerType: data.providerType,
        configuration: JSON.parse(data.configuration),
        createdAt: data.createdAt
    }
}
const fromForm = (data: ChannelForm): Channel => {
    return {
        id: data.id,
        name: data.name,
        providerType: data.providerType,
        configuration: JSON.stringify(data.configuration),
        createdAt: data.createdAt
    }
}

const DEFAULT_FORM: ChannelForm = {
    id: "",
    name: "",
    providerType: "EmailSmtp",
    configuration: {}
};

const ChannelFormSheet = ({ open, onOpenChange, initialData, onSubmit }: ChannelFormSheetProps) => {
    const isEdit = !!initialData;
    const [formData, setFormData] = useState<ChannelForm>(DEFAULT_FORM);

    // Reset or Populate on Open
    useEffect(() => {
        if (open) {
            setFormData(initialData ? { ...toForm(initialData) } : { ...DEFAULT_FORM });
        }
    }, [open, initialData]);

    const handleConfigChange = (key: string, value: string) => {
        setFormData(prev => ({
            ...prev,
            configuration: { ...prev.configuration, [key]: value }
        }));
    };

    const handleProviderChange = (val: ProviderType) => {
        // Reset config when switching providers to avoid junk data
        setFormData(prev => ({
            ...prev,
            providerType: val,
            configuration: {}
        }));
    };

    // --- Dynamic Sub-Forms ---
    const renderConfigurationFields = () => {
        switch (formData.providerType) {
            case 'EmailSmtp':

                return (
                    <>
                        <div className="space-y-2">
                            <Label>Host</Label>
                            <Input
                                // placeholder="alerts@company.com"
                                value={formData.configuration.host || ''}
                                onChange={e => handleConfigChange('host', e.target.value)}
                            />
                        </div>
                        <div className="space-y-2">
                            <Label>Port</Label>
                            <Input
                                // placeholder="alerts@company.com"
                                value={formData.configuration.port || ''}
                                onChange={e => handleConfigChange('port', e.target.value)}
                            />
                        </div>
                        <div className="space-y-2">
                            <Label>UserName</Label>
                            <Input
                                // placeholder="alerts@company.com"
                                value={formData.configuration.username || ''}
                                onChange={e => handleConfigChange('username', e.target.value)}
                            />
                        </div>
                        <div className="space-y-2">
                            <Label>Password</Label>
                            <Input
                                type="password"
                                // placeholder="alerts@company.com"
                                value={formData.configuration.password || ''}
                                onChange={e => handleConfigChange('password', e.target.value)}
                            />
                        </div>
                        <div className="space-y-2">
                            <Label>Recipient Emails (To)</Label>
                            <Input
                                placeholder="alerts@company.com"
                                value={formData.configuration.to_addresses || ''}
                                onChange={e => handleConfigChange('to_addresses', e.target.value)}
                            />
                        </div>
                        <div className="space-y-2">
                            <Label>From Address</Label>
                            <Input
                                placeholder="[Urgent]"
                                value={formData.configuration.from_address || ''}
                                onChange={e => handleConfigChange('from_address', e.target.value)}
                            />
                        </div>
                    </>
                );

            case 'SLACK':
                return (
                    <>
                        <div className="space-y-2">
                            <Label>Webhook URL</Label>
                            <Input
                                type="password"
                                placeholder="https://hooks.slack.com/services/..."
                                value={formData.configuration.webhook_url || ''}
                                onChange={e => handleConfigChange('webhook_url', e.target.value)}
                            />
                        </div>
                        <div className="space-y-2">
                            <Label>Target Channel (Optional)</Label>
                            <Input
                                placeholder="#alerts"
                                value={formData.configuration.channel || ''}
                                onChange={e => handleConfigChange('channel', e.target.value)}
                            />
                        </div>
                    </>
                );

            case 'GchatWebhook':
                return (
                    <div className="space-y-2">
                        <Label>Google Chat Webhook URL</Label>
                        <Input
                            // type="password"
                            placeholder="https://chat.googleapis.com/..."
                            value={formData.configuration.webhook_url || ''}
                            onChange={e => handleConfigChange('webhook_url', e.target.value)}
                        />
                    </div>
                );

            default:
                return <div className="text-sm text-muted-foreground">Select a provider type.</div>;
        }
    };

    return (

        <Sheet open={open} onOpenChange={onOpenChange}>
            {/* flex-col + h-full is mandatory on the parent */}
            <SheetContent className="w-[400px] bg-white border-l p-0 flex flex-col h-full">

                {/* HEADER: Fixed height */}
                <SheetHeader className="p-6 shrink-0 border-b">
                    <SheetTitle>{isEdit ? "Edit Channel" : "New Channel"}</SheetTitle>
                    <SheetDescription>Configure a notification destination.</SheetDescription>
                </SheetHeader>

                {/* CONTENT: This div takes all available space and scrolls */}
                <div className="flex-1 overflow-y-auto">
                    <div className="p-6 space-y-6">

                        {/* Common Fields */}
                        <div className="space-y-4">
                            <div className="space-y-2">
                                <Label>Channel ID</Label>
                                <Input
                                    value={formData.id}
                                    onChange={e => setFormData({...formData, id: e.target.value})}
                                    disabled={isEdit}
                                    placeholder="e.g. eng_slack_alerts"
                                    className={isEdit ? "bg-slate-100" : ""}
                                />
                            </div>

                            <div className="space-y-2">
                                <Label>Friendly Name</Label>
                                <Input
                                    value={formData.name}
                                    onChange={e => setFormData({...formData, name: e.target.value})}
                                    placeholder="Engineering Team Alerts"
                                />
                            </div>

                            <div className="space-y-2">
                                <Label>Provider Type</Label>
                                <Select
                                    value={formData.providerType}
                                    onValueChange={(val) => handleProviderChange(val as ProviderType)}
                                >
                                    <SelectTrigger>
                                        <SelectValue />
                                    </SelectTrigger>
                                    <SelectContent>
                                        <SelectItem value="EmailSmtp">
                                            <div className="flex items-center gap-2"><Mail className="w-4 h-4"/> Email Smtp</div>
                                        </SelectItem>
                                        <SelectItem value="GchatWebhook">
                                            <div className="flex items-center gap-2"><MessageSquare className="w-4 h-4"/> Google Chat</div>
                                        </SelectItem>
                                    </SelectContent>
                                </Select>
                            </div>
                        </div>

                        <Separator />

                        {/* Dynamic Configuration Area */}
                        <div className="space-y-4 p-4 bg-slate-50 rounded-md border border-slate-100">
                            <h4 className="text-xs font-bold uppercase text-muted-foreground tracking-wider mb-2">
                                {formData.providerType} Configuration
                            </h4>
                            {renderConfigurationFields()}
                        </div>
                    </div>
                </div>

                {/* FOOTER: Fixed height, pinned to bottom */}
                <SheetFooter className="p-6 shrink-0 border-t bg-slate-50">
                    <Button variant="outline" onClick={() => onOpenChange(false)}>Cancel</Button>
                    <Button
                        onClick={() => { onSubmit(fromForm(formData)); onOpenChange(false); }}
                        className="bg-blue-600 hover:bg-blue-700"
                    >
                        <Save className="w-4 h-4 mr-2" /> Save Channel
                    </Button>
                </SheetFooter>

            </SheetContent>
        </Sheet>


    );
};

export default ChannelFormSheet;
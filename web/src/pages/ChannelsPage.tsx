import {useEffect, useState} from 'react';
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Plus, Edit, Trash2, Mail, Hash, MessageSquare } from "lucide-react";
import {type Channel, type ProviderType} from '@/lib/types';
import ChannelFormSheet from "@/components/sheets/channel-form-sheet.tsx";
import {get, post, put} from "@/lib/fetcher.ts";
import {toast} from "sonner";

type ChannelsListResponse = {
    channels: Channel[];
}

const ChannelsPage = () => {
    const [sheetOpen, setSheetOpen] = useState(false);
    const [editingChannel, setEditingChannel] = useState<Channel | null>(null);

    const [channels, setChannels] = useState<Channel[]>([]);
    const [loading, setLoading] = useState(true);

    useEffect(() => {
        loadChannels();
    }, []);

    const loadChannels = async () => {
        setLoading(true);
        const data = await get<ChannelsListResponse>('/channels');
        console.log('data is' + data);
        if (data.isOk()) {
            setChannels(data.get().channels);
        } else {
            console.error("Failed to load channels", data.failure());
            toast.error("Could not load channels." + data.failure().message);
        }
        setLoading(false);
    };

    const handleCreate = () => {
        setEditingChannel(null);
        setSheetOpen(true);
    };

    const handleEdit = (channel: Channel) => {
        setEditingChannel(channel);
        setSheetOpen(true);
    };

    const handleFormSubmit = async (data: Channel) => {
        if (editingChannel) {
            let result = await put<Channel, Channel>('/channels', data);
            console.log(result);
            setChannels(channels.map(c => c.id === data.id ? data : c));
            toast.success("Channel config updated.");
        } else {
            let result = await post<Channel, Channel>('/channels', data);
            console.log(result);
            if (result.isOk()) {
                setChannels([...channels, data]);
                toast.success("New channel created.");
            } else {
                toast.error("Could not create channel.");
            }
        }
    };

    const getIcon = (type: ProviderType) => {
        switch (type) {
            case 'EmailSmtp': return <Mail className="w-5 h-5 text-red-500" />;
            case 'SLACK': return <Hash className="w-5 h-5 text-purple-500" />;
            case 'GchatWebhook': return <MessageSquare className="w-5 h-5 text-green-600" />;
        }
    };

    return (
        <div className="space-y-6">
            <div className="flex justify-between items-center">
                <div>
                    <h2 className="text-lg font-bold">Notification Channels</h2>
                    <p className="text-sm text-muted-foreground">Manage external integrations for alerts.</p>
                </div>
                <Button onClick={handleCreate} className="bg-blue-600 hover:bg-blue-700"><Plus className="w-4 h-4 mr-2" />Add Channel</Button>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                {!loading && channels.map((channel) => (
                    <Card key={channel.id} className="group hover:border-blue-300 transition-colors">
                        <CardHeader className="pb-3 flex flex-row items-center justify-between space-y-0">
                            <div className="flex items-center gap-3">
                                <div className="p-2 bg-slate-50 rounded-md border">
                                    {getIcon(channel.providerType)}
                                </div>
                                <div>
                                    <CardTitle className="text-base">{channel.name}</CardTitle>
                                    <CardDescription className="text-xs font-mono mt-0.5">{channel.id}</CardDescription>
                                </div>
                            </div>
                        </CardHeader>
                        <CardContent>
                            <div className="text-sm text-muted-foreground bg-slate-50 p-2 rounded border border-dashed mb-4 truncate">
                                {/* Simple preview of config */}
                                {JSON.stringify(channel.configuration).slice(0, 40)}...
                            </div>
                            <div className="flex justify-end gap-2">
                                <Button variant="ghost" size="sm" onClick={() => handleEdit(channel)}><Edit className="w-4 h-4 text-gray-500" /></Button>
                                <Button variant="ghost" size="sm" className="hover:text-red-600"><Trash2 className="w-4 h-4" /></Button>
                            </div>
                        </CardContent>
                    </Card>
                ))}
            </div>

            <ChannelFormSheet
                open={sheetOpen}
                onOpenChange={setSheetOpen}
                initialData={editingChannel}
                onSubmit={handleFormSubmit}
            />
        </div>
    );
};

export default ChannelsPage;
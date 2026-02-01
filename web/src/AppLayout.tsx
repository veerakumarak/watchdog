import { Outlet, NavLink } from 'react-router-dom';
import {
    LayoutDashboard,
    Settings,
    Radio,
    Activity,
    // LogOut,
    LucideHistory
} from "lucide-react";
import { cn } from "@/lib/utils";

const SidebarItem = ({ to, icon: Icon, label }: { to: string, icon: any, label: string }) => {
    return (
        <NavLink
            to={to}
            className={({ isActive }) => cn(
                "flex items-center gap-3 px-3 py-2 rounded-md text-sm font-medium transition-colors",
                isActive
                    ? "bg-blue-50 text-blue-700"
                    : "text-slate-600 hover:bg-slate-100 hover:text-slate-900"
            )}
        >
            <Icon className="w-4 h-4" />
            {label}
        </NavLink>
    );
};

const AppLayout = () => {
    return (
        <div className="flex h-screen bg-slate-50">
            {/* SIDEBAR */}
            <aside className="w-64 bg-white border-r flex flex-col h-full shrink-0">

                {/* Logo Area */}
                <div className="h-16 flex items-center px-6 border-b">
                    <div className="flex items-center gap-2 font-bold text-xl text-slate-800">
                        <Activity className="w-6 h-6 text-blue-600" />
                        Watchdog
                    </div>
                </div>

                {/* Navigation Links */}
                <div className="flex-1 py-6 px-3 space-y-1">
                    <SidebarItem to="/jobs" icon={LayoutDashboard} label="Job Configurations" />
                    <SidebarItem to="/channels" icon={Radio} label="Channels" />
                    <SidebarItem to="/settings" icon={Settings} label="System Settings" />
                    <SidebarItem to="/history" icon={LucideHistory} label="Execution History" />
                </div>

                {/* Footer / User Area */}
                {/*<div className="p-4 border-t">*/}
                {/*    <button className="flex items-center gap-2 text-sm text-muted-foreground hover:text-red-600 transition-colors w-full px-2">*/}
                {/*        <LogOut className="w-4 h-4" /> Sign Out*/}
                {/*    </button>*/}
                {/*</div>*/}
            </aside>

            {/* MAIN CONTENT AREA */}
            <main className="flex-1 overflow-auto">
                <div className="container max-w-6xl mx-auto py-8 px-6">
                    {/* Renders the current page based on the URL */}
                    <Outlet />
                </div>
            </main>
        </div>
    );
};

export default AppLayout;
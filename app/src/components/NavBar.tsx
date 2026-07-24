import React, { useState } from "react";

import { cn, useTransitionNavigate } from "../utils";
import { useWebSocket } from "../providers/WebSocketProvider";

import {
  Menu,
  Blinds,
  Images,
  Settings,
  LogOut,
  ShieldCogCorner,
} from "lucide-react";
import { Role } from "../types";

type NavItemT = {
  label: string;
  route?: string;
  icon: any;
  minRole?: Role;
};

function NavItem({
  onClick,
  item,
  isExpanded,
}: {
  onClick?: any;
  item: NavItemT;
  isExpanded: boolean;
}) {
  return (
    <a
      className="flex items-center px-5 py-3 text-muted-foreground hover:text-accent-foreground hover:bg-accent transition-colors group cursor-pointer"
      title={!isExpanded ? item.label : undefined}
      onClick={onClick}
    >
      <div className="text-current group-hover:scale-110 transition-transform">
        {item.icon}
      </div>

      <span
        className={cn(
          "ml-4 font-medium text-sm whitespace-nowrap transition-opacity duration-300",
          isExpanded ? "opacity-100" : "opacity-0 overflow-hidden w-0",
        )}
      >
        {item.label}
      </span>
    </a>
  );
}

const topNavItems: NavItemT[] = [
  {
    label: "Dashboard",
    route: "/dashboard",
    icon: <Blinds size={20} strokeWidth={1.5} className="shrink-0" />,
  },
  {
    label: "Gallery",
    route: "/gallery",
    icon: <Images size={20} strokeWidth={1.5} className="shrink-0" />,
  },
];

const bottomNavItems: NavItemT[] = [
  {
    label: "Admin Settings",
    route: "/admin",
    minRole: Role.Admin,
    icon: <ShieldCogCorner size={20} strokeWidth={1.5} className="shrink-0" />,
  },
  {
    label: "Settings",
    route: "/settings",
    icon: <Settings size={20} strokeWidth={1.5} className="shrink-0" />,
  },
];

export function NavBar() {
  const [isExpanded, setIsExpanded] = useState(false);

  const { user, logOut } = useWebSocket();
  const navigate = useTransitionNavigate();

  const navVisibility = (item: NavItemT) =>
    item.minRole === undefined || (user?.role || Role.User) >= item.minRole;

  const logOutBtn = () => {
    logOut();
    navigate("/");
  };

  const onRoute = (path: string) => {
    navigate(path);
  };

  return (
    <nav
      className={cn(
        "fixed top-0 left-0 h-screen bg-card/60 border-l-2 border-border flex flex-col select-none transition-all duration-300 ease-in-out",
        isExpanded ? "w-56" : "w-16",
      )}
    >
      <div className="flex flex-col flex-1 pt-4">
        <button
          onClick={() => setIsExpanded(!isExpanded)}
          className="flex items-center px-5 py-3 text-muted-foreground hover:text-accent-foreground hover:bg-accent transition-colors focus:outline-none"
        >
          <Menu size={20} strokeWidth={1.5} className="shrink-0" />
        </button>

        <div className="mt-8 flex flex-col gap-2">
          {topNavItems.filter(navVisibility).map((item) => (
            <NavItem
              key={item.label}
              item={item}
              onClick={() => item.route && onRoute(item.route)}
              isExpanded={isExpanded}
            />
          ))}
        </div>
      </div>

      <div className="flex flex-col gap-2 pb-6">
        {bottomNavItems.filter(navVisibility).map((item) => (
          <NavItem
            key={item.label}
            onClick={() => item.route && onRoute(item.route)}
            item={item}
            isExpanded={isExpanded}
          />
        ))}

        <NavItem
          item={{
            label: "Log Out",
            icon: (
              <LogOut
                size={20}
                strokeWidth={1.5}
                className="rotate-180 shrink-0"
              />
            ),
          }}
          onClick={logOutBtn}
          isExpanded={isExpanded}
        />
      </div>
    </nav>
  );
}

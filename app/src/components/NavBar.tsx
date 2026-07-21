import React, { useState } from "react";

import { cn, useTransitionNavigate } from "../utils";
import { useWebSocket } from "../providers/WebSocketProvider";

import { Menu, Blinds, Images, Settings, LogOut } from "lucide-react";

function NavItem({
  onClick,
  item,
  isExpanded,
}: {
  onClick?: any;
  item: any;
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

const topNavItems = [
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

const bottomNavItems = [
  {
    label: "Settings",
    icon: <Settings size={20} strokeWidth={1.5} className="shrink-0" />,
  },
];

export function NavBar() {
  const [isExpanded, setIsExpanded] = useState(false);

  const { disconnect } = useWebSocket();
  const navigate = useTransitionNavigate();

  const logOut = () => {
    localStorage.removeItem("auth_token");
    disconnect();
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
          {topNavItems.map((item, index) => (
            <NavItem
              key={index}
              item={item}
              onClick={() => onRoute(item.route)}
              isExpanded={isExpanded}
            />
          ))}
        </div>
      </div>

      <div className="flex flex-col gap-2 pb-6">
        {bottomNavItems.map((item, index) => (
          <NavItem key={index} item={item} isExpanded={isExpanded} />
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
          onClick={logOut}
          isExpanded={isExpanded}
        />
      </div>
    </nav>
  );
}

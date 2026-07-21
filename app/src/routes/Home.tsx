import React, { useEffect, useState } from "react";

import { BASE_URL, useTransitionNavigate } from "../utils";

import { LoaderCircle } from "lucide-react";

import Login from "../components/home/Login";
import Signup from "../components/home/Signup";
import CMA from "../components/home/CMA";

export default function Home() {
  const [doesAccountsExist, setDoesAccountsExist] = useState<boolean | null>(
    null,
  );
  const [view, setView] = useState<"login" | "signup">("login");

  const navigate = useTransitionNavigate();

  const handleAuth = (token: string) => {
    localStorage.setItem("auth_token", token);
    navigate("/dashboard");
  };

  useEffect(() => {
    fetch(`${BASE_URL}/does-any-user-exist`)
      .then((r) => r.json())
      .then(setDoesAccountsExist);
  }, []);

  return (
    <main className="relative flex min-h-svh items-center justify-center overflow-hidden px-4">
      <div className="relative w-full max-w-sm">
        <div className="flex flex-col gap-6 rounded-2xl border border-border bg-card/60 p-6 shadow-2xl shadow-black/40 backdrop-blur">
          <div className="flex flex-col items-center text-center gap-1.5">
            <h1 className="text-2xl font-semibold tracking-tight text-foreground">
              {doesAccountsExist === false
                ? "Setup Master Account"
                : view === "login"
                  ? "Sign In"
                  : "Create Account"}
            </h1>

            <p className="text-sm text-muted-foreground">
              {doesAccountsExist === false
                ? "Create the initial administrator account"
                : "Please enter your details to continue"}
            </p>
          </div>

          {doesAccountsExist === null && (
            <div className="flex w-full items-center justify-center py-8">
              <LoaderCircle
                className="animate-spin text-muted-foreground"
                size={32}
              />
            </div>
          )}

          {doesAccountsExist === true && (
            <>
              {view === "login" ? (
                <Login onAuth={handleAuth} />
              ) : (
                <Signup onAuth={handleAuth} />
              )}

              <div className="text-center text-sm text-muted-foreground mt-2">
                {view === "login"
                  ? "Don't have an account? "
                  : "Already have an account? "}

                <button
                  type="button"
                  onClick={() => setView(view === "login" ? "signup" : "login")}
                  className="hover:cursor-pointer text-primary font-medium hover:underline outline-none transition-colors"
                >
                  {view === "login" ? "Sign up" : "Sign in"}
                </button>
              </div>
            </>
          )}

          {doesAccountsExist === false && <CMA onAuth={handleAuth} />}
        </div>
      </div>
    </main>
  );
}

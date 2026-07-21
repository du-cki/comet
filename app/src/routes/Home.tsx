import React, { useEffect, useState } from "react";

import { BASE_URL, useTransitionNavigate } from "../utils";

import { LoaderCircle } from "lucide-react";

import Card from "../components/common/Card";

import Login from "../components/home/Login";
import Signup from "../components/home/Signup";
import CMA from "../components/home/CMA";

export default function Home() {
  const [doesAnyAccountExist, setDoesAccountsExist] = useState<boolean | null>(
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
    <main className="flex min-h-svh items-center justify-center overflow-hidden px-4">
      <div className="w-full max-w-sm">
        <Card className="space-y-6 p-6 rounded-2xl shadow-2xl shadow-black/40 backdrop-blur">
          <div className="text-center space-y-1.5">
            <h1 className="text-2xl font-semibold tracking-tight text-foreground">
              {doesAnyAccountExist === false
                ? "Setup Master Account"
                : view === "login"
                  ? "Sign In"
                  : "Create Account"}
            </h1>

            <p className="text-sm text-muted-foreground">
              {doesAnyAccountExist === false
                ? "Create the initial administrator account"
                : "Please enter your details to continue"}
            </p>
          </div>

          {doesAnyAccountExist === null && (
            <div className="flex w-full items-center justify-center py-8">
              <LoaderCircle
                className="animate-spin text-muted-foreground"
                size={32}
              />
            </div>
          )}

          {doesAnyAccountExist === true && (
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

          {doesAnyAccountExist === false && <CMA onAuth={handleAuth} />}
        </Card>
      </div>
    </main>
  );
}

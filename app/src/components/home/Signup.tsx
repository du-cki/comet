import React, { SyntheticEvent, useState } from "react";

import Button from "../common/Button";
import Input from "../common/Input";

import { BASE_URL } from "../../utils";

type Props = {
  onAuth: (token: string) => void;
};

export default function Signup({ onAuth }: Props) {
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<false | string>(false);

  const onSubmit = async (e: SyntheticEvent<HTMLFormElement>) => {
    try {
      e.preventDefault();

      setIsLoading(true);
      setError(false);

      const formData = new FormData(e.currentTarget);
      const formValues = Object.fromEntries(formData.entries());

      const req = await fetch(`${BASE_URL}/register`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(formValues),
      });

      if (!req.ok) {
        setError(await req.text());
        return;
      }

      const resp = await req.json();
      onAuth(resp["token"]);
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <form onSubmit={onSubmit}>
      <div className="space-y-4">
        <Input
          label="Name"
          name="name"
          type="text"
          required
          disabled={isLoading}
        />
        <Input
          label="Email"
          name="email"
          type="email"
          required
          disabled={isLoading}
        />
        <Input
          label="Password"
          name="password"
          type="password"
          required
          disabled={isLoading}
        />

        {error && <h1 className="text-sm text-destructive">{error}</h1>}
      </div>

      <Button
        type="submit"
        className="mt-6 w-full"
        variant="primary"
        isLoading={isLoading}
      >
        Create Account
      </Button>
    </form>
  );
}

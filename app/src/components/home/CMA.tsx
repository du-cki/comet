import React, { SyntheticEvent, useState } from "react";

import { api } from "../../client";

import Button from "../common/Button";
import Input from "../common/Input";

type Props = {
  onAuth: (token: string) => void;
};

export default function CMA({ onAuth }: Props) {
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<false | string>(false);

  const onSubmit = async (e: SyntheticEvent<HTMLFormElement>) => {
    try {
      e.preventDefault();

      setIsLoading(true);
      setError(false);

      const formData = new FormData(e.currentTarget);
      const formValues = Object.fromEntries(formData.entries());

      const { name, email, password } = formValues;

      const req = await api.register({
        name: name as string,
        email: email as string,
        password: password as string,
      });

      if (!req.ok) {
        setError(await req.text());
        return;
      }

      // TODO: instead of this,
      // maybe show a little onboarding before sending them to the dashboard?
      // like configuring the server?
      const resp = await req.json();
      onAuth(resp["token"]);
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <form onSubmit={onSubmit}>
      <div className="space-y-4">
        <Input label="Name" name="name" required disabled={isLoading} />

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
        Create Master Account
      </Button>
    </form>
  );
}

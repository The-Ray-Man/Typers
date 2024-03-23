import { Code, Text, Textarea } from "@mantine/core";

import Card from "./Card";

type ErrorProps = {
  error: string;
};

const Error = ({ error }: ErrorProps) => {
  return (
    <Card title="Error">
      <Code color="red.2" block miw={"100%"}>
        {error}
      </Code>
    </Card>
  );
};

export default Error;

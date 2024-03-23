import { PropsWithChildren } from "react";
import {
  Card as MantineCard,
  Center,
  Text,
  Stack,
  Divider,
} from "@mantine/core";

type CardProps = PropsWithChildren & {
  title: string;
};

const Card = ({ children, title }: CardProps) => {
  return (
    <MantineCard withBorder miw={"650"}>
      <Stack align="center">
        <Center>
          <Text>{title}</Text>
        </Center>
        {children}
      </Stack>
    </MantineCard>
  );
};

export default Card;

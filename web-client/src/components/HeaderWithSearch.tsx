import { Button, Container, Flex, Input, Select } from "@mantine/core";
import { Form } from "@remix-run/react";

export default function HeaderWithSearch() {
  return (
    <Flex
      justify="space-between"
      align="center"
    >
      <h1>PoE Ledger</h1>

      <Form method="get">
        <Flex
          justify="flex-end"
          align="center"
          gap="md"
        >
          <Input
            name="get"
            type="text"
            placeholder="Divine Orb"
          />

          <Select
            defaultValue={"Sanctum"}
            placeholder="Leagues"
            data={[
              { value: 'Sanctum', label: 'Sanctum' },
            ]}
          />

          <input type="submit" hidden></input>
        </Flex>

      </Form>

    </Flex>

  )
}

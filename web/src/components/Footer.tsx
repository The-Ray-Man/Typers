import { Center, Flex, SimpleGrid, Text } from "@mantine/core";
import { IconBrandGithub, IconHeart } from "@tabler/icons-react";


const Footer = () => {
    return (
            <SimpleGrid cols={3}>
                <div></div>
                <Center>

        <Text pe={"xs"}>Made with ❤️ by TecTrixer and The-Ray-Man</Text>
                </Center>
                <Flex justify={"end"}>

        <IconBrandGithub size={24} />
                </Flex>
            </SimpleGrid>
    );
    }
export default Footer;

import { Center, Code, Stack, Text} from "@mantine/core"
import { TreeTS } from "FMFP";
import { MathJax } from "better-react-mathjax";
import Card from "./Card";

type TreeProps = {
    mathjax: String;
}

const Tree = ({mathjax}:TreeProps) => {
    return (
        <Card title="Tree">
            <MathJax>
                {"\\(" + mathjax +"\\)"}
       </MathJax>
        </Card>
    )
    
}

export default Tree;
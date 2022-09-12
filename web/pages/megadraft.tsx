import { Box } from "@mui/system";
import { MegadraftEditor, editorStateFromRaw } from "megadraft";
import 'megadraft/dist/css/megadraft.css'
import { NextPage } from "next";

const Megadraft: NextPage = () => {
    return (
        <Box
            sx={{
                mx: "20px"
            }}
        >
            <MegadraftEditor

            />
        </Box>)
}

export default Megadraft
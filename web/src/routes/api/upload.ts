import fsSync from "fs";
import fs from "fs/promises";
import type { APIEvent } from "solid-start/api";
import { json } from "solid-start/api";
import multipart from "parse-multipart-data"
import path from "path"

export async function POST(event: APIEvent) {
    console.log("Called upload")
    const new_headers: NodeJS.Dict<string | string[]> = {};

    for (const header of event.request.headers.entries()) {
        new_headers[header[0]] = header[1]
    }

    console.log(new_headers)

    const body = event.request.body
    const reader = await body?.getReader().read();
    const content = reader?.value?.toString();
    console.log(content)

    if (typeof content !== "undefined") {
        if (!fsSync.existsSync("gradivo")) {
            await fs.mkdir("gradivo")
        }

        const boundary = content.split("\n")[0].trim().slice(2);
        const parts = multipart.parse(Buffer.from(content), boundary);
        console.log({ boundary, parts })

        parts.forEach(async part => {
            if (part.filename) {
                const file_path = path.join("gradivo", part.filename)
                await fs.writeFile(file_path, part.data);
            }
        })
    }


    return json({
        message: "File saves successfully"
    });
}
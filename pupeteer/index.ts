import puppeteer from 'puppeteer';
import fs from 'fs/promises';

const delay = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));

(async () => {
    const browser = await puppeteer.launch({
        headless: false
    });

    const page = await browser.newPage();
    page.on('console', msg => console.log(msg.text()));

    const file = await fs.readFile("./links.txt");
    const links = file.toString().split('\n');
    links.shift()

    for (const link of links) {
        console.log("url", link)

        await Promise.all([
            page.waitForNavigation(), // The promise resolves after navigation has finished
            page.goto(link), // Clicking the link will indirectly cause a navigation
        ]);

        // writes only one function
        // console log parsed math originalText
        await page.exposeFunction('readfile', async filePath => {
            return new Promise((resolve, reject) => {
                fs.writeFile(filePath)
            });
        });
        const result = await page.evaluate(() => {
            /* const getCircularReplacer = () => {
                const seen = new WeakSet();
                return (key, value) => {
                    if (typeof value === "object" && value !== null) {
                        if (seen.has(value)) {
                            return;
                        }
                        seen.add(value);
                    }
                    return value;
                };
            }

            return Promise.resolve(JSON.stringify(window.MathJax.Hub.getAllJax(), getCircularReplacer())); */
        });

        console.log(result)
    };


    await browser.close();
})();
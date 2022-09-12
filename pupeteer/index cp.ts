import puppeteer from 'puppeteer';
import fs from 'fs/promises';

const delay = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));

(async () => {
    const browser = await puppeteer.launch({
        headless: false
    });

    const page = await browser.newPage();

    const file = await fs.readFile("./links.txt");
    const links = file.toString().split('\n');
    links.shift()

    await Promise.all([
        page.waitForNavigation(), // The promise resolves after navigation has finished
        page.goto(links[0]), // Clicking the link will indirectly cause a navigation
    ]);

    let MathJax = {
        test: "yes"
    };
    await page.exposeFunction("getMathJax", () => {
        const getCircularReplacer = () => {
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

        return JSON.stringify(MathJax.Hub.getAllJax(), getCircularReplacer());
    })

    for (const link of links) {
        console.log("url", link)

        await Promise.all([
            page.waitForNavigation(), // The promise resolves after navigation has finished
            page.goto(link), // Clicking the link will indirectly cause a navigation
        ]);

        await page.evaluate(async function () {
            const myMathJax = await getMathJax();
            MathJax = myMathJax;
        })

        console.log(MathJax)
    };


    await browser.close();
})();
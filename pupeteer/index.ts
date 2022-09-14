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
    links.shift();

    let index = 0;
    await fs.rm("maths", { recursive: true });
    await fs.mkdir("maths");

    for (const link of links) {
        console.log("url", link)

        await Promise.all([
            page.waitForNavigation(), // The promise resolves after navigation has finished
            page.goto(link), // Clicking the link will indirectly cause a navigation
        ]);


        const result = await page.evaluate(function () {
            let arr = window.MathJax.Hub.getAllJax();
            let result = ""

            for (const obj of arr) {
                result += obj.originalText + " a a a a "
            }

            return Promise.resolve(result)
        });

        console.log(result)

        result.split("#!#").join("\n")

        await fs.writeFile(`maths/course ${index}.txt`, result);
        await delay(1000);
        index++
    };


    await browser.close();
})();
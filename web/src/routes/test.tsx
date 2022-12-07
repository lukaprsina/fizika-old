import { Tab, TabGroup, TabList, TabPanel } from "solid-headless";
import type { Component } from "solid-js";
import TinyMCE from "~/components/TinyMCE";

const Test: Component = () => {
    return (
        <TabGroup
            defaultValue={0}
            horizontal={false}
        >{({ isSelected, isActive }) => (
            <>
                <TabList>
                    <Tab value={0}>Hello</Tab>
                    <Tab value={1}>TinyMCE</Tab>
                </TabList>
                <div>
                    <TabPanel value={0}>
                        <p>Hi</p>
                    </TabPanel>
                    <TabPanel value={1}>
                        <TinyMCE authorized={true} visible={isActive(1)} />
                    </TabPanel>
                </div>
            </>
        )}</TabGroup>
    )

}

export default Test;
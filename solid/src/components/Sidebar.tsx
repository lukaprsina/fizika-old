import { Component, ParentComponent } from "solid-js";
import { A } from "solid-start";

export const Sidebar: ParentComponent = (props) => {
    return <div>{props.children}</div>
}

type NavbarItemType = {
    text: string;
    href?: string;
}

export const SidebarItem: Component<NavbarItemType> = (props) => {
    return (
        <A class="block p-3 hover:bg-[#f8f9fa] rounded-md" href={props.href ?? props.text}>
            {props.text}
        </A>
    )

}
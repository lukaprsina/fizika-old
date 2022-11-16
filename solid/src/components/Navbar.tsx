import { Component, ParentComponent } from "solid-js";
import { A } from "solid-start";

export const Navbar: ParentComponent = (props) => {
    return <div>{props.children}</div>
}

type NavbarItemType = {
    text: string;
    href?: string;
}

export const NavbarItem: Component<NavbarItemType> = (props) => {
    return (
        <A class="block" href={props.href ?? props.text}>
            {props.text}
        </A>
    )

}
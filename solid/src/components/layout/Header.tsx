import { Component, Show } from "solid-js";
import { A } from "solid-start";
import { useEditToggle } from "~/routes/(home)";


type HeaderType = {
    topic?: string;
}

const Header: Component<HeaderType> = (props) => {
    const editToggle = useEditToggle();

    return (
        <div
            class="flex justify-between items-center h-16 w-full px-4"
        >
            <A href="/" class="m-2">
                <div class="flex items-center h-16">
                    <img
                        src="/images/scnm-logo.jpg"
                        alt="Logo šolskega centra Novo mesto"
                        class="h-3/4 mr-4"
                    />
                    <span>Fizika</span>
                </div>
            </A>
            <Show when={props.topic}>
                <A href={encodeURI("/" + props.topic)}>{props.topic}</A>
            </Show>
            <div>
                <input
                    type="checkbox"
                    class="mr-3"
                    onChange={() => editToggle.change(!editToggle.edit())}
                />
                <label>Edit</label>
            </div>
        </div>
    )
}

export default Header;
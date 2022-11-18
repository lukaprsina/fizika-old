import { ParentComponent } from "solid-js";

const Content: ParentComponent = (props) => {
    return (
        <div class="max-w-5xl mx-auto">
            {props.children}
        </div>
    )
}

export default Content
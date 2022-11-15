import { Component } from "solid-js"
import { useParams } from "solid-start"

const Course: Component = () => {
    const params = useParams();
    return <p>{params.course}</p>
}

export default Course
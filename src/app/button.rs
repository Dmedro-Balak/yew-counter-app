use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct CounterButtonProps {
    text: String,
}

#[function_component]
pub fn CounterButton(props: &CounterButtonProps) -> Html {}
